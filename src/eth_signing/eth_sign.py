from signers import SignWithKey
from eth_prive_action import SignEthPrivateAction

def sign(network_id, ethereum_address, method, request_path, body, timestamp, private_key):
    # GANACHE_ADDRESS = '0x90F8bf6A479f320ead074411a4B0e7944Ea8c9C1'
    # GANACHE_PRIVATE_KEY = (
    #     '0x4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d'
    # )
    # PARAMS = {
    #     'method': 'POST',
    #     'request_path': 'v3/test',
    #     'body': '{}',
    #     'timestamp': '2021-01-08T10:06:12.500Z',
    # }

    eth_signer = SignWithKey(private_key)
    signer = SignEthPrivateAction(eth_signer, network_id)
    # eth_signer = SignWithKey(GANACHE_PRIVATE_KEY)
    # signer = SignEthPrivateAction(eth_signer, 1)
    # signature = signer.sign(
    #     GANACHE_ADDRESS,
    #     **PARAMS
    # )
    signature = signer.sign(
            ethereum_address,
            method=method,
            request_path=request_path,
            body=body,
            timestamp=timestamp
        )
    return signature