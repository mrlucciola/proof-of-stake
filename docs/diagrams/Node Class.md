# Node

Node has a REST API to receive signed user transactions. Transactions must be signed if they want to be submitted to the txn pool.

Node is responsible for peer to peer communications. It is a submodule within the node. P2P is used to communicate aross the pool of nodes.

Node maintains the Blockchain. The Blockchain can create and add new blocks. Transactions are taken from the transaction pool and added to the block.