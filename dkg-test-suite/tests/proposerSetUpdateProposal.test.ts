
/*
 * Copyright 2022 Webb Technologies Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
import 'jest-extended';
import {BLOCK_TIME} from '../src/constants';
import {
  localChain,
  polkadotApi,
  executeAfter,
  signatureBridge,
} from './utils/util';
import {jest} from "@jest/globals";
import { Option } from '@polkadot/types';
import { HexString } from '@polkadot/util/types';
import { BigNumber, ethers } from 'ethers';

jest.setTimeout(10000 * BLOCK_TIME);

describe('Validator Node Test', () => {
  test('proposer set update test', async () => {
    const provider = localChain.provider();
    
    const chainIdType = polkadotApi.createType('WebbProposalsHeaderTypedChainId', {
      None: 0,
    });
    const key = {
      ProposerSetUpdateProposal: 1,
    };
    const proposal = await polkadotApi.query.dKGProposalHandler.signedProposals(chainIdType, key);
    const value = new Option(polkadotApi.registry, 'DkgRuntimePrimitivesProposal', proposal);
    expect(value.isSome).toBeTrue();
    const dkgProposal = value.unwrap().toJSON() as {
      signed: {
        kind: 'ProposerSetUpdate';
        data: HexString;
        signature: HexString;
      };
    };
    const bridgeSide = await signatureBridge.getBridgeSide(localChain.chainId);
    const contract = bridgeSide.contract;
    const isSignedByGovernor = await contract.isSignatureFromGovernor(
      dkgProposal.signed.data,
      dkgProposal.signed.signature
    );
    expect(isSignedByGovernor).toBeTrue();

    const proposalData = dkgProposal.signed.data.slice(2);
    const proposerSetRoot = `0x${proposalData.slice(0, 64)}`;
    const averageSessionLength = BigNumber.from(`0x${proposalData.slice(64, 80)}`);
    const numOfProposers = BigNumber.from(`0x${proposalData.slice(80, 88)}`);
    const proposalNonce = BigNumber.from(`0x${proposalData.slice(88, 96)}`);

    const tx = await contract.updateProposerSetData(
      proposerSetRoot,
      averageSessionLength,
      numOfProposers,
      proposalNonce,
      dkgProposal.signed.signature
    );
    await expect(tx.wait()).toResolve();

    const contractProposerSetRoot = await bridgeSide.contract.proposerSetRoot();
    expect(proposerSetRoot).toEqual(contractProposerSetRoot);

    const contractAverageSessionLength = await bridgeSide.contract.averageSessionLengthInMillisecs();
    expect(averageSessionLength).toEqual(BigNumber.from(contractAverageSessionLength));

    const contractNumOfProposers = await bridgeSide.contract.numOfProposers();
    expect(numOfProposers).toEqual(BigNumber.from(contractNumOfProposers));

    const contractProposalNonce = await bridgeSide.contract.proposerSetUpdateNonce();
    expect(proposalNonce).toEqual(BigNumber.from(contractProposalNonce));

    // // Now the proposer set root on the contract has been updated

    let proposerAccounts = await polkadotApi.query.dKGProposals.externalProposerAccounts.entries();
    let accounts = new Array();
    for (let i = 0; i<proposerAccounts.length; i++) {
      let account = proposerAccounts[i][1];
      accounts.push(account.toHuman());
    }

    let hash0 = ethers.utils.keccak256(accounts[0]);
    let hash1 = ethers.utils.keccak256(accounts[1]);
    let hash2 = ethers.utils.keccak256(accounts[2]);
    let hash3 = ethers.utils.keccak256('0x00');

    let hash01 = ethers.utils.keccak256(hash0.concat(hash1.slice(2)));
    let hash23 = ethers.utils.keccak256(hash2.concat(hash3.slice(2)));
    let root = ethers.utils.keccak256(hash01.concat(hash23.slice(2)));
    
    await provider.send('evm_increaseTime', [600]);
    await provider.send("evm_mine");

    // bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice
    const signer0 = new ethers.Wallet('0x79c3b7fc0b7697b9414cb87adcb37317d1cab32818ae18c0e97ad76395d1fdcf');

    // bottom drive obey lake curtain smoke basket hold race lonely fit walk//Bob
    const signer1 = new ethers.Wallet('0xf8d74108dbe199c4a6e4ef457046db37c325ba3f709b14cabfa1885663e4c589');

    // bottom drive obey lake curtain smoke basket hold race lonely fit walk//Charlie
    const signer2 = new ethers.Wallet('0xcb6df9de1efca7a3998a8ead4e02159d5fa99c3e0d4fd6432667390bb4726854');

    const voteProposer0 = 
    {
      leafIndex: 0, 
      siblingPathNodes:[hash1, hash23], 
      proposedGovernor: '0x1111111111111111111111111111111111111111'
    };

    await contract.connect(provider.getSigner(signer0.address)).voteInFavorForceSetGovernor(voteProposer0);

    const voteProposer1 = 
    {
      leafIndex: 1, 
      siblingPathNodes:[hash0, hash23], 
      proposedGovernor: '0x1111111111111111111111111111111111111111'
    };

    await contract.connect(provider.getSigner(signer1.address)).voteInFavorForceSetGovernor(voteProposer1);

    expect('0x1111111111111111111111111111111111111111').toEqual(await contract.governor());
  });

  afterAll(async () => {
    await executeAfter();
  });
});