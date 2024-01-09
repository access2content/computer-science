# 1. Install geth locally
- Download [Go-Ethereum](https://github.com/ethereum/go-ethereum) or geth.
- Run the `make all` command to install geth and other utilities on the system

Copy the geth and other binary to /usr/bin folder for running the commands
```shell
sudo cp ./build/bin/* /usr/bin
```
# 2. Create folder for nodes
Create folders for nodes in the folder `~/.ethereum` for finding it easily later on. For this example, they are inside the `nodes` folder named 0, 1, and 2.

# 3. Create 2 accounts
For this example, the folder `~/.ethereum/nodes/0/keystore` is used. You can use your own folder. There are various ways to create accounts in geth, but the recommended one is by using `clef`.

```shell
clef newaccount --keystore ~/.ethereum/nodes/0/keystore
```

When prompted, specify a password and remember it. Note that the password should not be <10 characters.

# 4. Create Genesis Block
Add a genesis.json config file as follows in the root folder for all nodes:
```json
{
    "config": {
        "chainId": 15,
        "homesteadBlock": 0,
        "eip150Block": 0,
        "eip155Block": 0,
        "eip158Block": 0
    },
    "difficulty": "400000",
    "gasLimit": "2100000",
    "alloc": {
        "[Account #1 Address]": { "balance": "1000000000000000000" },
        "[Account #2 Address]": { "balance": "2000000000000000000" },
    }
}
```
Replace the Account #1 Address and #2 address with the accounts that were generated earlier.

Then, generate the genesis block for all nodes as follows:
```shell
geth --datadir ~/.ethereum/nodes/0 init ~/.ethereum/nodes/0/genesis.json
```
# Resources
- [How to deploy a local private Ethereum blockchain](https://medium.com/datawallet-blog/how-to-deploy-a-local-private-ethereum-blockchain-c2b497f068f4)
- [Getting started with Geth](https://geth.ethereum.org/docs/getting-started)
- [Connecting to Consensus Client](https://geth.ethereum.org/docs/getting-started/consensus-clients)