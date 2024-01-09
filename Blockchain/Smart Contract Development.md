For Smart Contracts, [Solidity](https://docs.soliditylang.org/en/v0.8.23/) is the language of choice. The documentation for learning Solidity is pretty good and is a great place to start.

The first thing to do is to setup the compiler on your system so that you can run/test your solidity code.

One of the best places that I found to start learning Solidity is the [Solidity by Example](https://docs.soliditylang.org/en/v0.8.23/solidity-by-example.html) on the solidity lang website. There are a few contracts written there which seem to be a great starting point.

# Install the Compiler
Based on the OS used, the compiler can be installed locally according to the installation instructions found on the [Solidity website](https://docs.soliditylang.org/en/v0.8.23/installing-solidity.html).

# Write a Smart Contract
Instead of writing a smart contract from scratch, we can use an already existing smart contract from the [Solidity by Example](https://docs.soliditylang.org/en/v0.8.23/solidity-by-example.html) page.

For following example, I will use the voting contract as example.
# Compiling a contract
The compiler is `solc` which can be run on the source file or files. To compile a file, just pass it to the compiler.
```bash
solc voting.sol
```

This however will not create any output. There are various output formats available, so one can specify the output they want. By default, the compiler will output to the console.

```bash
solc --bin voting.sol
```

The options available for output types include:
- bin
- asm
- ast-compact-json
- abi

To see a list of all available output types, use `solc --help` command and navigate to the **Output Components** section.

To save the output to file(s), you need to specify the output directory using the `-o` flag:
```shell
solc --bin --asm -o output voting.sol
solc --bin --asm -o . voting.sol
```

This will save the specified output file types to the directory specified after the `o` flag.

More details can be found in the [using the compiler](https://docs.soliditylang.org/en/v0.8.23/using-the-compiler.html) section on solidity's website.

# Deploying a Contract
Now that a Smart Contract has been written and compiled, it needs to be deployed on a blockchain network. This network can be anywhere. For development and testing purposes, it is wiser to do it locally. So, now, we need to install a Blockchain network locally for deployment and interaction.