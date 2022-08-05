# solanaWeb3
这是一个非常好的和酷的异步项目，为期一周的项目适用于想要使用 Solana 进行破解的好奇开发者。您将学习一些 Rust，编写并部署一个 Solana 程序，并将其全部连接回一个 React web3 应用程序，任何拥有 Solana 钱包的人都可以与之交互！

## 一 什么是Solana
- 链上合约程序
  - Solana合约代码，是运行在solana公链上的一段链上程序代码，类似于以太坊中的智能合约;
  - 链上合约代码的执行，是由组成整个区块链节点，通过共识打包的形式进行交易运行;

- 账户类型
  - 在Solana中链上合约是无状态的，而不像以太坊的智能合约，合约数据是直接存储在合约账户中;
  - 在Solana账户模型中，允许用户在自己的账户创建多个子账户，并通过链上合约程序与子账户直接进行交互。
    - 注意: 链上合约程序并不保存用户数据，只负责实现合约业务逻辑，通过链上程序的调用来完成子账户之间的交互;

- Solana与Ethereum比较
  - 每个公链平台，都有各自的优点和缺点，只需要选择适合业务场景的技术即可。
  - Solana以运行速度快、Gas费低；
  - 以太坊平台安全健壮、生态强大、技术资源多，但缺点是Gas费高、运行效率慢。

- 跨链技术
  - 比如eth、solana、bnb都是不同的链，跨链就是他们之间可以相互转换;


## 二 web应用连接到Solana钱包
### 1、安装 Phantom Wallet 钱包并构建连接按钮
### 2、构建gif网格
```javascript
/*
 * We are going to be using the useEffect hook!
 */
import React, { useEffect, useState } from 'react';
import twitterLogo from './assets/twitter-logo.svg';
import './App.css';
// Change this up to be your Twitter if you want.
const TWITTER_HANDLE = '_buildspace';
const TWITTER_LINK = `https://twitter.com/${TWITTER_HANDLE}`;
const App = () => {
  /*
   * This function holds the logic for deciding if a Phantom Wallet is
   * connected or not
   */
  const [walletAddress, setWalletAddress] = useState(null);
  const checkIfWalletIsConnected = async () => {
    try {
      const { solana } = window;
      if (solana) {
        if (solana.isPhantom) {
          console.log('Phantom wallet found!');
          const response = await solana.connect({ onlyIfTrusted: true });
          setWalletAddress(response.publicKey.toString());
          console.log(
            'Connected with Public Key:',
            response.publicKey.toString()
          );
        }
      } else {
        console.log('Solana object not found! Get a Phantom Wallet ');
      }
    } catch (error) {
      console.error(error);
    }
  };

  const connectWallet = async () => {
    const { solana } = window;
    if (solana) {
      const response = await solana.connect();
      console.log('Connected with Public Key:', response.publicKey.toString());
    }
  };

  const TEST_GIFS = [
    'https://i.giphy.com/media/eIG0HfouRQJQr1wBzz/giphy.webp',
    'https://media3.giphy.com/media/L71a8LW2UrKwPaWNYM/giphy.gif?cid=ecf05e47rr9qizx2msjucl1xyvuu47d7kf25tqt2lvo024uo&rid=giphy.gif&ct=g',
    'https://media4.giphy.com/media/AeFmQjHMtEySooOc8K/giphy.gif?cid=ecf05e47qdzhdma2y3ugn32lkgi972z9mpfzocjj6z1ro4ec&rid=giphy.gif&ct=g',
    'https://i.giphy.com/media/PAqjdPkJLDsmBRSYUp/giphy.webp'
  ];

  const [inputValue, setInputValue] = useState('');

  const onInputChange = (event) => {
    const { value } = event.target;
    setInputValue(value);
  };

  const [gifList, setGifList] = useState([]);

  // to难过过表单添加新的图片信息，需要同步修改gifList状态变量以及清空输入框
  const sendGif = async () => {
    if (inputValue.length > 0) {
      console.log('Gif link:', inputValue);
      setGifList([...gifList, inputValue]);
      setInputValue('');
    } else {
      console.log('Empty input. Try again.');
    }
  };

  const renderConnectedContainer = () => (
    <div className="connected-container">
      <form
        onSubmit={(event) => {
          event.preventDefault();
          sendGif();
        }}
      >
        <input type="text" placeholder="Enter gif link!" value={inputValue} onChange={onInputChange}/>
        <button type="submit" className="cta-button submit-gif-button">Submit</button>
      </form>
      <div className="gif-grid">
        {gifList.map((gif) => (
          <div className="gif-item" key={gif}>
            < img src={gif} alt={gif} />
          </div>
        ))}
      </div>
    </div>
  );

  const renderNotConnectedContainer = () => (
    <button
      className="cta-button connect-wallet-button"
      onClick={connectWallet}
    >
      Connect to Wallet
    </button>
  );

  /*
   * When our component first mounts, let's check to see if we have a connected
   * Phantom Wallet
   */
  // react钩子初始化
  useEffect(() => {
    if (walletAddress) {
      // TODO: 查询Solana链上数据
      let result = TEST_GIFS; //说明：在本小节暂时以测试图片数据代替链上数据，之后会在另的章节进行代码实现
      console.log('Fetching GIF list…');
      setGifList(result);
    };
    const onLoad = async () => {
      await checkIfWalletIsConnected();
    };
    
    window.addEventListener('load', onLoad);
    return () => window.removeEventListener('load', onLoad);
  }, [walletAddress]);
  
  return (
    <div className="App">
      <div className={walletAddress ? 'authed-container' : 'container'}>
         <div className="header-container">
          <p className="header">🖼 GIF Portal</p >
          <p className="sub-text">
            View your GIF collection in the metaverse 
          </p >
          {!walletAddress ? renderNotConnectedContainer() : walletAddress}
          {walletAddress && renderConnectedContainer()}
        </div>
        <div className="footer-container">
          <img alt="Twitter Logo" className="twitter-logo" src={twitterLogo} />
          <a
            className="footer-text"
            href={TWITTER_LINK}
            target="_blank"
            rel="noreferrer"
          >{`built on @${TWITTER_HANDLE}`}</a>
        </div>
      </div>
    </div>
  );
};
export default App;
```

## 三 编写Solana程序
- solana项目流程
  - 1.用户提交自己上传的图片数据
  - 2.将图片数据保存至Solana链上程序中
  - 3.从Solana链上查询用户图片数据

### 1、搭建本地Solana底链环境
- 在Solana生态中，链上程序需要使用Rust语言进行编写。So，需要搭建rust环境;

#### 1.搭建Rust环境（https://doc.rust-lang.org/book/ch01-01-installation.html）
  - rustup --version 查看版本
  - rustc --version 查看是否能正常编译
  - cargo -- version 查看管理依赖版是否正常
  
#### 2.安装Solana
  - 1）安装solana命令（https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool）
    - 跟着步骤安装
    - solana -V  查看版本检测是否安装成功
    - 添加环境变量
    ```shell
    downloading v1.10.32 installer
  ✨ 1.10.32 initialized
    Adding 
    export PATH="/Users/zhangxiangyu/.local/share/solana/install/active_release/bin:$PATH" to /Users/zhangxiangyu/.profile
    Adding 
    export PATH="/Users/zhangxiangyu/.local/share/solana/install/active_release/bin:$PATH" to /Users/zhangxiangyu/.zprofile
    Adding 
    export PATH="/Users/zhangxiangyu/.local/share/solana/install/active_release/bin:$PATH" to /Users/zhangxiangyu/.bash_profile
    ```
  - 2）配置solana参数（需要将solana参数配置成本地链网络）
    - solana config set --url localhost
    - solana config get
  - 3）运行本地链网
    - solana-test-validator  可以再本地快速运行一条solana链网络。在Solana正式网络中，其实主要是一些验证节点来保证整个网络的安全运行。实际上在本地运行也是一个验证节点;
  
### 2、构建工程项目
#### 1.依赖安装
- 1）安装Node、Yarn以及Mocha
  - yarn: 类似于Npm包管理器
    - npm install yarn
  - mocha: Solana用力验证测试框架工具
    - npm install -g mocha
- 2）安装脚手架工具
  - Anchor: Solana生态中的一个脚手架工具，类似于以太坊开发生态中的Hardhat脚手架，可以非常方便地为我们初始化项目环境，
  并且支持项目快速启动一条本地链，方便发布与验证solana程序;
    - cargo install --git https://github.com/project-serum/anchor --tag v0.24.2 anchor-cli --locked

#### 2.初始化项目工程
- 1）通过anchor初始化solana项目环境
```shell
anchor init my_epic_project --javascript
cd my_epic_project
```

#### 3.创建公私钥对
- 1）本地生成一个公私钥账户地址
  - solana-keygen new
- 2）查看所有地址   
  - solana address

#### 4.验证项目工程配置
- 1）anchor test
  - ① 编译Rust合约程序；
  - ② 使用solana-test-validator启动本地链，并将我们的Rust合约程序发布至本地Solana网络；
  - ③ 对链上Rust合约程序进行函数调用测试。

### 3、编写第一个Solana链上合约
### 4、Solana合约编码 [存储基本类型数据]
### 5、Solana合约编码 [存储自定义结构体数据]


## 四 部署+连接Solana程序到Web应用程序
### 1、将程序部署到devnet
### 2、从Web应用程序调用部署的程序
### 3、向Solana程序提交GIF