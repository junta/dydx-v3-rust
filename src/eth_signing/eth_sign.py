from signers import SignWithKey
from eth_prive_action import SignEthPrivateAction
from onboarding_action import SignOnboardingAction

def sign(network_id, ethereum_address, method, request_path, body, timestamp, private_key):
    eth_signer = SignWithKey(private_key)
    signer = SignEthPrivateAction(eth_signer, network_id)
    signature = signer.sign(
            ethereum_address,
            method=method,
            request_path=request_path,
            body=body,
            timestamp=timestamp
        )
    return signature

def sign_onboarding(network_id, ethereum_address, action, private_key):
    eth_signer = SignWithKey(private_key)
    signer = SignOnboardingAction(eth_signer, network_id)
    signature = signer.sign(
            ethereum_address,
            action=action
        )
    return signature