const MusicManagerTest = artifacts.require("MusicManagerTest");

/*
 * uncomment accounts to access the test accounts made available by the
 * Ethereum client
 * See docs: https://www.trufflesuite.com/docs/truffle/testing/writing-tests-in-javascript
 */
contract("MusicManagerTest", function (/* accounts */) {
  it("should assert true", async function () {
    await MusicManagerTest.deployed();
    return assert.isTrue(true);
  });
});
