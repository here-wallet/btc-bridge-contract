const GAS = "200000000000000";
const { Account, utils: { format: { parseNearAmount } } } = nearAPI;


const vec_to_dict = (vector) => {
  let res = {};
  vector.forEach(val => {
    res[val[0]] = val[1];
  })
  return res;
}

describe('Token', function () {
  let near;
  let contract;
  let accountId;

  beforeAll(async function () {
    console.log('nearConfig', nearConfig);
    near = await nearlib.connect(nearConfig);
    accountId = nearConfig.contractName;

    contract = await near.loadContract(accountId, {
      viewMethods: [''],
      changeMethods: ['play_game',],
      sender: accountId
    });

    await contract.play_game({}, GAS);
  });

  describe('counter', function () {
    it('can make bet', async function () {
      await contract.make_bet({ "bt": 1 }, GAS, parseNearAmount('1.123'));
      const deposit_x2 = await contract.get_deposit_x2();

      console.log(vec_to_dict(deposit_x2));

      // expect(endCounter).toEqual(startCounter + 1);
    });
  });
});