import { Worker, NEAR, NearAccount } from "near-workspaces";
import anyTest, { TestFn } from "ava";

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

test.beforeEach(async (t) => {
  // Init the worker and start a Sandbox server
  const worker = await Worker.init();

  const root = worker.rootAccount;

  // define users
  const vault = await root.createSubAccount("vault", {
    initialBalance: NEAR.parse("30 N").toJSON(),
  });

  const alice = await root.createSubAccount("alice", {
    initialBalance: NEAR.parse("30 N").toJSON(),
  });

  const bob = await root.createSubAccount("bob", {
    initialBalance: NEAR.parse("30 N").toJSON(),
  });

  const contract = await root.createSubAccount("contract", {
    initialBalance: NEAR.parse("30 N").toJSON(),
  });
  
  // Deploy the contract.
  await contract.deploy(process.argv[2]);

  // Initialize vault
  await contract.call(contract, "init", {vault: vault.accountId})

  // Save state for test runs, it is unique for each test
  t.context.worker = worker;
  t.context.accounts = { root, contract, vault, alice, bob };
});

test.afterEach(async (t) => {
  // Stop Sandbox server
  await t.context.worker.tearDown().catch((error) => {
    console.log("Failed to stop the Sandbox:", error);
  });
});

test("sends deposits to the vault", async (t) => {
  const { contract, alice, vault } = t.context.accounts;

  const balance = await vault.balance();
  const available = parseFloat(balance.available.toHuman());

  await alice.call(contract, "deposit", {}, { attachedDeposit: NEAR.parse("1 N").toString() });

  const new_balance = await vault.balance();
  const new_available = parseFloat(new_balance.available.toHuman());

  t.is(new_available, available + 1 - 0.001);
});

test("records the deposit", async (t) => {
  const { contract, bob } = t.context.accounts;

  await bob.call(contract, "deposit", {}, { attachedDeposit: NEAR.parse("2 N").toString() });

  const deposit: Deposit = await contract.view("get_deposit_for_account", { account_id: bob.accountId });

  t.is(deposit.account_id, bob.accountId);
  t.is(deposit.total_amount, NEAR.parse("2 N").toString());
});

class Deposit{
  account_id: string = "";
  total_amount: string = "";
}