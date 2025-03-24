## 简介

该脚本是一个基于 @autonomys/auto-utils 和 @autonomys/auto-xdm 的工具，用于在 Auto Taurus 测试网络上将代币从一个账户转账到另一个 EVM 地址。它通过 transferToDomainAccount20Type 方法实现跨域转账。

## 环境要求

在运行此脚本之前，请确保您已安装以下工具和库：

Node.js (建议版本 >= 16.x)

npm包管理工具

## 配置

创建一个 .env 文件并添加以下内容：

```env
PRIVATE_KEY=您的私钥/助词词
WS_ENDPOINT=WebSocket 节点地址
EVM_ADDRESS=目标 EVM 地址
```

- PRIVATE_KEY：源账户的私钥，用于签名交易。
- WS_ENDPOINT：Auto 网络的 WebSocket 端点，例如：wss://taurus.autonomy.io/ws。
- EVM_ADDRESS：接收方的 EVM 地址。

## 安装依赖

``` bash
npm install @autonomys/auto-utils @autonomys/auto-xdm dotenv
```
## 使用说明

- 确保已正确配置 .env 文件。
- 本脚本会接连指定的Node节点，节点运行需要增加参数`--allow-private-ips --rpc-methods unsafe` 才会允许外部链接，如果你自己运行节点，务必增加这个参数


## 运行脚本：

``` bash
node consensus-to-evm.js
```

##  功能说明

- 激活钱包：通过 activateWallet 方法连接到 Auto 网络并加载账户。
- 验证账户：检查是否加载了账户，并打印源地址以便调试。
- 跨域转账：使用 transferToDomainAccount20Type 函数从源账户向目标 EVM 地址转账。

## 示例输出
成功连接到网络后会打印源地址：
``` css
st8GMptMgxQNNKviNWMzXKQHvQactTX3iMbSxUpdmetxxx //发送源钱包地址
0x935cc82b28998273f6eab67ee31337f7f2xxx126bed27e8c2e5c56 //交易哈希
```

## 注意事项

请确保您的账户中有足够的代币以支付交易费用。转账金额单位为AI3,可以在.env文件中编辑。

## 特别鸣谢
本脚本得到了AI工具的大力支持，包含Grok,Chatgpt.

## 联系
如果您在使用过程中遇到任何问题，请联系开发者或提交 issue。