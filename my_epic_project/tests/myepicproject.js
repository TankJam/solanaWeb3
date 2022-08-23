const anchor = require("@project-serum/anchor");
const { SystemProgram } = anchor.web3;

const main = async() => {
  console.log("🚀 Starting test...");
  // 调用anchor.setProvider代码，并初始化Solana链网配置以及钱包帐户信息；具体要导入的配置，可以参见项目根目录下的Anchor.toml文件
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // 在测试文件中调用的命名方式为：anchor.workspace.Myepicproject
  const program = anchor.workspace.Myepicproject;
 
  // 1）通过anchor给对象生成新账户
  const baseAccount = anchor.web3.Keypair.generate();

  // 合约函数的调用其实都是通过program.rpc.startStuffOff()来触发执行的
  // 其实是通过区块链验证节点对区块共识成功之后进行触发调用的
  // 2）构建合约函数参数并调用
  let tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log("📝 Your transaction signature", tx);

  // Fetch data from the account
  // 首次打印当前计数
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count', account.totalGifs.toString());

  // 记录计数
  await program.rpc.addGif("https://pic.cnblogs.com/avatar/1297508/20200401185001.png", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  // 调用addGif后，再次查看计数
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count', account.totalGifs.toString());

  console.log('👀 GIF List', account.gifList)
}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();