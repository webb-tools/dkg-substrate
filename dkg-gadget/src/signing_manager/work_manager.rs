use crate::{
	async_protocols::remote::AsyncProtocolRemote, debug_logger::DebugLogger, utils::SendFuture,
	NumberFor,
};
use dkg_primitives::types::DKGError;
use parking_lot::RwLock;
use sp_api::BlockT;
use std::{
	collections::{HashSet, VecDeque},
	hash::{Hash, Hasher},
	pin::Pin,
	sync::Arc,
};
use sync_wrapper::SyncWrapper;

#[derive(Clone)]
pub struct WorkManager<B: BlockT> {
	inner: Arc<RwLock<WorkManagerInner<B>>>,
	// for now, use a hard-coded value for the number of tasks
	max_tasks: usize,
	logger: DebugLogger,
	to_handler: tokio::sync::mpsc::UnboundedSender<[u8; 32]>,
}

pub struct WorkManagerInner<B: BlockT> {
	pub currently_signing_proposals: HashSet<Job<B>>,
	pub enqueued_signing_proposals: VecDeque<Job<B>>,
}

impl<B: BlockT> WorkManager<B> {
	pub fn new(logger: DebugLogger, max_tasks: usize) -> Self {
		let (to_handler, mut rx) = tokio::sync::mpsc::unbounded_channel();
		let this = Self {
			inner: Arc::new(RwLock::new(WorkManagerInner {
				currently_signing_proposals: HashSet::new(),
				enqueued_signing_proposals: VecDeque::new(),
			})),
			max_tasks,
			logger,
			to_handler,
		};

		let this_worker = this.clone();
		let handler = async move {
			let job_receiver_worker = this_worker.clone();
			let logger = job_receiver_worker.logger.clone();

			let job_receiver = async move {
				while let Some(unsigned_proposal_hash) = rx.recv().await {
					job_receiver_worker.logger.info_signing(format!(
						"[worker] Received job {:?}",
						unsigned_proposal_hash
					));
					job_receiver_worker.poll();
				}
			};

			let periodic_poller = async move {
				let mut interval = tokio::time::interval(std::time::Duration::from_millis(200));
				loop {
					interval.tick().await;
					this_worker.poll();
				}
			};

			tokio::select! {
				_ = job_receiver => {
					logger.error_signing("[worker] job_receiver exited");
				},
				_ = periodic_poller => {
					logger.error_signing("[worker] periodic_poller exited");
				}
			}
		};

		tokio::task::spawn(handler);

		this
	}

	/// Pushes the task, but does not necessarily start it
	pub fn push_task(
		&self,
		unsigned_proposal_hash: [u8; 32],
		handle: AsyncProtocolRemote<NumberFor<B>>,
		task: Pin<Box<dyn SendFuture<'static, ()>>>,
	) -> Result<(), DKGError> {
		let mut lock = self.inner.write();
		let job = Job {
			task: Arc::new(RwLock::new(Some(task.into()))),
			handle,
			proposal_hash: Arc::new(unsigned_proposal_hash),
			tasks: self.inner.clone(),
			logger: self.logger.clone(),
		};
		lock.enqueued_signing_proposals.push_back(job);

		self.to_handler
			.send(unsigned_proposal_hash)
			.map_err(|_| DKGError::GenericError {
				reason: "Failed to send job to worker".to_string(),
			})
	}

	fn poll(&self) {
		// go through each task and see if it's done
		// finally, see if we can start a new task
		let mut lock = self.inner.write();
        // todo: do not just drop these tasks. Instead, check to see if any are stalled, and if so, restart them by pushing them to the front of the enqueued queue
		lock.currently_signing_proposals.retain(|job| !job.handle.is_done());

		// now, check to see if there is room to start a new task
		let tasks_to_start = self.max_tasks - lock.currently_signing_proposals.len();
		for _ in 0..tasks_to_start {
			if let Some(job) = lock.enqueued_signing_proposals.pop_front() {
				if let Err(err) = job.handle.start() {
					self.logger.error_signing(format!(
						"Failed to start job {:?}: {err:?}",
						job.proposal_hash
					));
				}
				let job_clone = job.clone();
				lock.currently_signing_proposals.insert(job);
				// run the task
				let task = async move {
					let task = job_clone.task.write().take().unwrap();
					task.into_inner().await
				};

				// Spawn the task. When it finishes, it will clean itself up
				let _ = tokio::task::spawn(task);
			}
		}
	}

	pub fn job_exists(&self, job: &[u8; 32]) -> bool {
		let lock = self.inner.read();
		lock.currently_signing_proposals.contains(job) ||
			lock.enqueued_signing_proposals.iter().any(|j| &*j.proposal_hash == job)
	}
}

#[derive(Clone)]
pub struct Job<B: BlockT> {
	// wrap in an arc to get the strong count for this job
	proposal_hash: Arc<[u8; 32]>,
	// a reference to the set of currently signing proposals. Used for deleting itself from the set
	tasks: Arc<RwLock<WorkManagerInner<B>>>,
	logger: DebugLogger,
	handle: AsyncProtocolRemote<NumberFor<B>>,
	task: Arc<RwLock<Option<SyncWrapper<Pin<Box<dyn SendFuture<'static, ()>>>>>>>,
}

impl<B: BlockT> std::borrow::Borrow<[u8; 32]> for Job<B> {
	fn borrow(&self) -> &[u8; 32] {
		&self.proposal_hash
	}
}

impl<B: BlockT> PartialEq for Job<B> {
	fn eq(&self, other: &Self) -> bool {
		self.proposal_hash == other.proposal_hash
	}
}

impl<B: BlockT> Eq for Job<B> {}

impl<B: BlockT> Hash for Job<B> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.proposal_hash.hash(state);
	}
}

impl<B: BlockT> Drop for Job<B> {
	fn drop(&mut self) {
		// There should only ever be two instances of a Job:
		// one inside the currently_enqueed set, and one inside the actual running task
		// Thus, if one of this gets dropped, we should remove it from the set
		if Arc::strong_count(&self.proposal_hash) == 2 {
			self.logger.info_signing(format!(
				"Will remove job {:?} from currently_signing_proposals",
				self.proposal_hash
			));
			self.tasks.write().currently_signing_proposals.remove(self);
			let _ = self.handle.shutdown("shutdown from Job::drop");
		}
	}
}
