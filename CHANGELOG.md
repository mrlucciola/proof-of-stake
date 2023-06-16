# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

## [0.3.0](https://github.com/mrlucciola/proof-of-stake/compare/v0.2.0...v0.3.0) (2023-06-16)


### Features

* [#000](https://github.com/mrlucciola/proof-of-stake/issues/000) add custom serialized blakehash impl ([4720413](https://github.com/mrlucciola/proof-of-stake/commit/4720413a304889fa850afad5ac1c3c2b824e691a))
* [#000](https://github.com/mrlucciola/proof-of-stake/issues/000) add license ([805a032](https://github.com/mrlucciola/proof-of-stake/commit/805a032068ec16cc2b01e930d34e12bc18a24fe0))
* [#105](https://github.com/mrlucciola/proof-of-stake/issues/105) add readme and update yarn config ([059c118](https://github.com/mrlucciola/proof-of-stake/commit/059c11854b016057e97cae6e152a56600dd17c26))
* [#107](https://github.com/mrlucciola/proof-of-stake/issues/107) - import tokio ([3e43ba3](https://github.com/mrlucciola/proof-of-stake/commit/3e43ba3d391c028d474a89bc94553e897a65a8e4))
* [#107](https://github.com/mrlucciola/proof-of-stake/issues/107) node p2p test add annotation ([a63639e](https://github.com/mrlucciola/proof-of-stake/commit/a63639e81e5bee24073d14e79d3e3c2d6e08d443))
* [#107](https://github.com/mrlucciola/proof-of-stake/issues/107) p2p- add listener with libp2p - large refactor ([07d698f](https://github.com/mrlucciola/proof-of-stake/commit/07d698fa9202d2cf1eb70cee8d0b006baf41d5d8))
* [#107](https://github.com/mrlucciola/proof-of-stake/issues/107) p2p- add peerid field and conversion ([b692ea6](https://github.com/mrlucciola/proof-of-stake/commit/b692ea6d768fcb103d21afab08abb5a6bf762d67))
* [#109](https://github.com/mrlucciola/proof-of-stake/issues/109) add block_header struct ([ce7a00e](https://github.com/mrlucciola/proof-of-stake/commit/ce7a00e0f7741b747426726f55b6fbdc4b1d96e9))
* [#109](https://github.com/mrlucciola/proof-of-stake/issues/109) add txn_header struct ([6b55622](https://github.com/mrlucciola/proof-of-stake/commit/6b55622ad0c34f83981985ed6e8d2f3c9775a783))
* [#109](https://github.com/mrlucciola/proof-of-stake/issues/109) refactor block and txn serialization for multiple usecases ([955ac72](https://github.com/mrlucciola/proof-of-stake/commit/955ac7292c492e89cfb9a1d3dd9e44c35243a1b3))
* [#29](https://github.com/mrlucciola/proof-of-stake/issues/29) create `Blockchain` struct ([022751f](https://github.com/mrlucciola/proof-of-stake/commit/022751f35462b15d7b765b9b20f1de7a9e3455ec))
* [#32](https://github.com/mrlucciola/proof-of-stake/issues/32) add `Block.is_signature_valid` method with test ([a952dda](https://github.com/mrlucciola/proof-of-stake/commit/a952dda2a591246bbc0330f545d064c7e609815b))
* [#32](https://github.com/mrlucciola/proof-of-stake/issues/32) add `genesis` method to `Block` ([c479938](https://github.com/mrlucciola/proof-of-stake/commit/c479938867595cc94d6e0b2f01d15e34a67561cb))
* [#32](https://github.com/mrlucciola/proof-of-stake/issues/32) add genesis method to `Blockchain` struct ([b7e010f](https://github.com/mrlucciola/proof-of-stake/commit/b7e010ffbc24550941e3c5805f4aaea4e047be71))
* [#37](https://github.com/mrlucciola/proof-of-stake/issues/37) create `Blockchain.add_block` method ([51700b9](https://github.com/mrlucciola/proof-of-stake/commit/51700b9356ba4f0af1d2f5e1e89349c1ece18ac2))
* [#40](https://github.com/mrlucciola/proof-of-stake/issues/40) add `accounts` struct ([61aeffe](https://github.com/mrlucciola/proof-of-stake/commit/61aeffe64f853197c07d9aeed29303262ac2ad54))
* [#41](https://github.com/mrlucciola/proof-of-stake/issues/41) add accounts `Acct` ([c3d3e44](https://github.com/mrlucciola/proof-of-stake/commit/c3d3e44974e7be0608e2b541c4d72e2ef6b78594))
* [#43](https://github.com/mrlucciola/proof-of-stake/issues/43) add acct to accounts map ([1853a7a](https://github.com/mrlucciola/proof-of-stake/commit/1853a7a79e61b625a9be9fe4bd85ddfcf812bd6e))
* [#54](https://github.com/mrlucciola/proof-of-stake/issues/54)  add tests and methods for incr and decr acct balance ([9a746db](https://github.com/mrlucciola/proof-of-stake/commit/9a746db01b6f7919b1f203fc053ac3f61136a77b))
* [#54](https://github.com/mrlucciola/proof-of-stake/issues/54) add `accounts` struct to `Blockchain` ([63080ff](https://github.com/mrlucciola/proof-of-stake/commit/63080ff0385043083825579c792a1c3a16e4fcee))
* [#54](https://github.com/mrlucciola/proof-of-stake/issues/54) add test for `blockchain.add_block()` ([3a1187f](https://github.com/mrlucciola/proof-of-stake/commit/3a1187f993bf273eed599be7cf80faceadcec7d5))
* [#54](https://github.com/mrlucciola/proof-of-stake/issues/54) mid commit ([5f8b0fb](https://github.com/mrlucciola/proof-of-stake/commit/5f8b0fb0cc13c27d81991eb3d22a936d59e3e1f0))
* [#54](https://github.com/mrlucciola/proof-of-stake/issues/54) successful test for execute_txn ([67855b1](https://github.com/mrlucciola/proof-of-stake/commit/67855b1dde1e88b3554c7ca015ff628aca208db6))
* [#60](https://github.com/mrlucciola/proof-of-stake/issues/60) add block signature validation for `add_block()`; fix tests; ([8677d88](https://github.com/mrlucciola/proof-of-stake/commit/8677d88e0317326ac4175318ef04a34d7e1d90c9))
* [#62](https://github.com/mrlucciola/proof-of-stake/issues/62) add `execute multiple txns()` ([70718b2](https://github.com/mrlucciola/proof-of-stake/commit/70718b247e8e4deb4d84d6e849c58b9431458dea))
* [#64](https://github.com/mrlucciola/proof-of-stake/issues/64) add `TxnPool.pop_txn()` method ([e360073](https://github.com/mrlucciola/proof-of-stake/commit/e360073ac7068e6d3f26272a529493669c70bd53))
* [#77](https://github.com/mrlucciola/proof-of-stake/issues/77) add Node dir in src and tests ([0885d24](https://github.com/mrlucciola/proof-of-stake/commit/0885d243ee29f6f0e55fbdf86e27c2da616e72c5))
* [#82](https://github.com/mrlucciola/proof-of-stake/issues/82) add startp2p ([c1ecdf4](https://github.com/mrlucciola/proof-of-stake/commit/c1ecdf47c112a6ad2a0fb171beb2d0770559f6e6))
* [#89](https://github.com/mrlucciola/proof-of-stake/issues/89) add p2p binary to run and listen to requests for testing ([5c6e75b](https://github.com/mrlucciola/proof-of-stake/commit/5c6e75b04594b6c5af551d8f424df0f08399e2dd))
* [#90](https://github.com/mrlucciola/proof-of-stake/issues/90) add from-string-impl to convert id to string for block-txn map ([4324b63](https://github.com/mrlucciola/proof-of-stake/commit/4324b638e0111997a931caabb2fdc172d3af3674))
* [#90](https://github.com/mrlucciola/proof-of-stake/issues/90) add libp2p ([01d1539](https://github.com/mrlucciola/proof-of-stake/commit/01d1539be54563c9cf3994dc86f5fd7dec70d9af))
* [#90](https://github.com/mrlucciola/proof-of-stake/issues/90) add txn types and fix tests ([7a5cd80](https://github.com/mrlucciola/proof-of-stake/commit/7a5cd809ea8c551eceacf24252acaf032fbede9a))
* [#90](https://github.com/mrlucciola/proof-of-stake/issues/90) remove blakehash from block ([4c82da9](https://github.com/mrlucciola/proof-of-stake/commit/4c82da979b264a3c02178d2ac2880c72ef2dd06e))
* [#90](https://github.com/mrlucciola/proof-of-stake/issues/90) remove unused methods on txn and block ([8d04141](https://github.com/mrlucciola/proof-of-stake/commit/8d04141e883b9e9a14ade98155c084c4e7d93ea8))
* [#90](https://github.com/mrlucciola/proof-of-stake/issues/90) replacing txnid type ([045ebeb](https://github.com/mrlucciola/proof-of-stake/commit/045ebebc41610c6e2269d27085c45d1565023a9d))
* [#90](https://github.com/mrlucciola/proof-of-stake/issues/90) secp-2-ed25519 ([27baf7b](https://github.com/mrlucciola/proof-of-stake/commit/27baf7b1d2b13f1680a44bb3ac3aded7bccb3d99))
* [#90](https://github.com/mrlucciola/proof-of-stake/issues/90) set block map and key to blockid ([beb2f3e](https://github.com/mrlucciola/proof-of-stake/commit/beb2f3eca13ae53b62ba2e2f811e1684b3a77fa4))
* [#90](https://github.com/mrlucciola/proof-of-stake/issues/90) set up new txn signing in wallet ([0f7ae07](https://github.com/mrlucciola/proof-of-stake/commit/0f7ae071e8b37cdf8df951ea127a84018037b950))
* [#90](https://github.com/mrlucciola/proof-of-stake/issues/90) update block interfaces ([31f1864](https://github.com/mrlucciola/proof-of-stake/commit/31f18641146b97579ef021657c33fbe2426443ce))


### Bug Fixes

* [#000](https://github.com/mrlucciola/proof-of-stake/issues/000) acct error handling; implement Display for txn type; fix formatting ([e3d444b](https://github.com/mrlucciola/proof-of-stake/commit/e3d444b7ab41b2259d9ef43be008ce08aea5cc28))
* [#000](https://github.com/mrlucciola/proof-of-stake/issues/000) moved /diagrams to /docs/diagrams ([e89cf81](https://github.com/mrlucciola/proof-of-stake/commit/e89cf811aa3c9f9817cccd480ec811012c744b1e))
* [#000](https://github.com/mrlucciola/proof-of-stake/issues/000) remove linter errors and unused connection test ([b7b66e1](https://github.com/mrlucciola/proof-of-stake/commit/b7b66e191b43fbe630fd6528599ac4fc8c27af1d))
* [#000](https://github.com/mrlucciola/proof-of-stake/issues/000) rename to_bytes to serialize; refactor txnheader ([f305be2](https://github.com/mrlucciola/proof-of-stake/commit/f305be241805423ad85f2669da29d32c4743593f))
* [#000](https://github.com/mrlucciola/proof-of-stake/issues/000) update gitignore and remove main.rs ([65648ff](https://github.com/mrlucciola/proof-of-stake/commit/65648ff4fe7745e6d4438f78c98b987c74992bb0))
* [#108](https://github.com/mrlucciola/proof-of-stake/issues/108) remove option types from node and p2p ([ad5c4cc](https://github.com/mrlucciola/proof-of-stake/commit/ad5c4cc3ebca00fa326ce3fe5768f30721112645))
* [#109](https://github.com/mrlucciola/proof-of-stake/issues/109) fix block and id calc_id and tests ([ef02262](https://github.com/mrlucciola/proof-of-stake/commit/ef02262c0bc8ce1a2560ff4003706374466a01ca))
* [#109](https://github.com/mrlucciola/proof-of-stake/issues/109) fix block, blockchain, and txn tests ([185c94d](https://github.com/mrlucciola/proof-of-stake/commit/185c94dd9de2cbbf3b767d8370bda82ea5150686))
* [#29](https://github.com/mrlucciola/proof-of-stake/issues/29) add tests ([012e988](https://github.com/mrlucciola/proof-of-stake/commit/012e9883cf8c3e13090c6d8480a53d59dc4fa2f7))
* [#32](https://github.com/mrlucciola/proof-of-stake/issues/32) more misc fixes to txn signature generation; all tests fixed ([84c02b2](https://github.com/mrlucciola/proof-of-stake/commit/84c02b266037fc31b17150ce523f25de909d0742))
* [#32](https://github.com/mrlucciola/proof-of-stake/issues/32) standardized access to internal `Block` variables ([4610d90](https://github.com/mrlucciola/proof-of-stake/commit/4610d9060dfee81780d59f718b2baba74b4eca7b))
* [#32](https://github.com/mrlucciola/proof-of-stake/issues/32) updated block signing standards ([3fc3df6](https://github.com/mrlucciola/proof-of-stake/commit/3fc3df6ebcf4ddd828fd195369fc5529c16feb08))
* [#33](https://github.com/mrlucciola/proof-of-stake/issues/33) rename block_height to blockheight ([23092bd](https://github.com/mrlucciola/proof-of-stake/commit/23092bdc9becd93b117d362ca411027e4f74e2f0))
* [#37](https://github.com/mrlucciola/proof-of-stake/issues/37) convert `hash` to `id` for `Block` ([5b5fdc4](https://github.com/mrlucciola/proof-of-stake/commit/5b5fdc47e3fb33c7a2e526b5d2db552dbf021925))
* [#37](https://github.com/mrlucciola/proof-of-stake/issues/37) convert `hash` to `id` for `Txn` ([d6fc506](https://github.com/mrlucciola/proof-of-stake/commit/d6fc506117ef18101a4f4b133fb4683c79bbda48))
* [#40](https://github.com/mrlucciola/proof-of-stake/issues/40) massive refactor; fix tests ([2d78f52](https://github.com/mrlucciola/proof-of-stake/commit/2d78f5263a4f94c3ded0335da36370d1a9045182))
* [#40](https://github.com/mrlucciola/proof-of-stake/issues/40) rename all instances of `Acct*` to `Account*` ([5f9b275](https://github.com/mrlucciola/proof-of-stake/commit/5f9b2754bad20294aa876b9d821db8d745534d66))
* [#40](https://github.com/mrlucciola/proof-of-stake/issues/40) update tests; refactor for organization and readability ([03c06d5](https://github.com/mrlucciola/proof-of-stake/commit/03c06d5c5bcd9b18b05d4493856d19ba7d880ee0))
* [#60](https://github.com/mrlucciola/proof-of-stake/issues/60) update `Block.is_signature_valid()` and `is_valid()` methods to accept pbkey ([2a513c6](https://github.com/mrlucciola/proof-of-stake/commit/2a513c6a36056bbf175e37c5daa26d35e8a21d20))
* [#64](https://github.com/mrlucciola/proof-of-stake/issues/64) fix txn_pool test for add_txn & associated error case ([38df4ae](https://github.com/mrlucciola/proof-of-stake/commit/38df4ae20b64e4400442380de53eb76cef72dd2b))
* [#74](https://github.com/mrlucciola/proof-of-stake/issues/74) resolve block signature issues with tests ([c253c44](https://github.com/mrlucciola/proof-of-stake/commit/c253c44f821af56316984b916c85664d3cc31263))
* [#90](https://github.com/mrlucciola/proof-of-stake/issues/90) fix blockchain signing and msg hashing issues ([f37cf87](https://github.com/mrlucciola/proof-of-stake/commit/f37cf8716bf4d59031f9d3344ee65c829346bada))
* [#90](https://github.com/mrlucciola/proof-of-stake/issues/90) fix txn tests by adding from from and partialeq impls on txn ([4f5fe75](https://github.com/mrlucciola/proof-of-stake/commit/4f5fe754ac2407d2bc3deb33f658e6c6406c35b8))
* [#90](https://github.com/mrlucciola/proof-of-stake/issues/90) remove errors from test ([54f88ee](https://github.com/mrlucciola/proof-of-stake/commit/54f88ee59be09db77a15546545f29375d9691387))
* [#94](https://github.com/mrlucciola/proof-of-stake/issues/94) fix broken txn tests ([f6a05b4](https://github.com/mrlucciola/proof-of-stake/commit/f6a05b4eb6be4456e5eb70fe00e2adc0c5889264))

## [0.2.0](https://github.com/mrlucciola/proof-of-stake/compare/v0.1.0...v0.2.0) (2022-11-29)


### Features

* [#13](https://github.com/mrlucciola/proof-of-stake/issues/13) add `block` diagram ([5e0003a](https://github.com/mrlucciola/proof-of-stake/commit/5e0003a5b6020d99c8d1d0e212e52760d0731187))
* [#13](https://github.com/mrlucciola/proof-of-stake/issues/13) add `blocks` functionality ([807dfeb](https://github.com/mrlucciola/proof-of-stake/commit/807dfeb82c07554c1a3be8be953fc517497500a6))
* [#16](https://github.com/mrlucciola/proof-of-stake/issues/16) change `TxnHash` to arr ([#17](https://github.com/mrlucciola/proof-of-stake/issues/17)) ([b55a921](https://github.com/mrlucciola/proof-of-stake/commit/b55a9216898bb15655af003929cf8d33aabd2b29))


### Bug Fixes

* [#15](https://github.com/mrlucciola/proof-of-stake/issues/15) remove failure lib ([18e82e5](https://github.com/mrlucciola/proof-of-stake/commit/18e82e555a8d856d80bbab32b9d7421b0d6289f4))
* [#20](https://github.com/mrlucciola/proof-of-stake/issues/20) add simple create-block test for `blocks` ([#21](https://github.com/mrlucciola/proof-of-stake/issues/21)) ([c90b949](https://github.com/mrlucciola/proof-of-stake/commit/c90b949042f1e8fe961c3948a627eb421296d1d0))
* [#22](https://github.com/mrlucciola/proof-of-stake/issues/22) add txn tests ([f03bc7f](https://github.com/mrlucciola/proof-of-stake/commit/f03bc7f27b5cc9cda7fd9a44c1dc75fb443fcab5))
* [#22](https://github.com/mrlucciola/proof-of-stake/issues/22) add wallet tests and clean up txn pool ([2122145](https://github.com/mrlucciola/proof-of-stake/commit/212214555449e3b1184b4d4dfe011dc8938fb276))
* [#22](https://github.com/mrlucciola/proof-of-stake/issues/22) migrate txn_pool tests ([f075020](https://github.com/mrlucciola/proof-of-stake/commit/f0750202e12f4347bb6682b5505578e994f67745))

## 0.1.0 (2022-11-22)


### Features

* add `does_txn_exist` method to `TxnPool` ([#7](https://github.com/mrlucciola/proof-of-stake/issues/7)) ([#8](https://github.com/mrlucciola/proof-of-stake/issues/8)) ([3b6558e](https://github.com/mrlucciola/proof-of-stake/commit/3b6558e83541e1f790cf5c48e1c429d7c102f3c6))
* add `remove_txn` (4) ([#6](https://github.com/mrlucciola/proof-of-stake/issues/6)) ([74a76a8](https://github.com/mrlucciola/proof-of-stake/commit/74a76a8988e62406c79ed449c932d13f7d916de9))

## 0.0.0 (2022-11-22)


### Features

* [#9](https://github.com/mrlucciola/proof-of-stake/issues/9) add commit hook and semver tools ([bef3d14](https://github.com/mrlucciola/proof-of-stake/commit/bef3d143275a89eed4e77c8051533ece510e1e18))
* [#9](https://github.com/mrlucciola/proof-of-stake/issues/9) add semver config - Conventional Changelog Configuration ([5dbede0](https://github.com/mrlucciola/proof-of-stake/commit/5dbede0ab34624f2ff0edfcafa057687772c6dab))
* add `does_txn_exist` method to `TxnPool` ([#7](https://github.com/mrlucciola/proof-of-stake/issues/7)) ([#8](https://github.com/mrlucciola/proof-of-stake/issues/8)) ([3b6558e](https://github.com/mrlucciola/proof-of-stake/commit/3b6558e83541e1f790cf5c48e1c429d7c102f3c6))
* add `remove_txn` (4) ([#6](https://github.com/mrlucciola/proof-of-stake/issues/6)) ([74a76a8](https://github.com/mrlucciola/proof-of-stake/commit/74a76a8988e62406c79ed449c932d13f7d916de9))
