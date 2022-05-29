from signers import SignWithKey
from eth_prive_action import SignEthPrivateAction
from onboarding_action import SignOnboardingAction
from web3 import Web3
import base64

def sign_private(network_id, ethereum_address, method, request_path, body, timestamp, private_key):
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

def derive_stark_private_key(signature):
    signature_int = int(signature, 16)
    hashed_signature = Web3.solidityKeccak(['uint256'], [signature_int])
    private_key_int = int(hashed_signature.hex(), 16) >> 5
    return hex(private_key_int)

def derive_secret(signature):
    signature_int = int(signature, 16)
    hashed_r_bytes = bytes(Web3.solidityKeccak(['uint256'], [signature_int]))
    secret_bytes = hashed_r_bytes[:30]
    secret = base64.urlsafe_b64encode(secret_bytes).decode()
    return secret

def derive_passphrase(signature):
    s_int = int(signature, 16)
    hashed_s_bytes = bytes(Web3.solidityKeccak(['uint256'], [s_int]))
    key_bytes = hashed_s_bytes[:16]
    passphrase_bytes = hashed_s_bytes[16:31]
    passphrase = base64.urlsafe_b64encode(passphrase_bytes).decode()
    return passphrase

def derive_key(signature):
    s_int = int(signature, 16)
    hashed_s_bytes = bytes(Web3.solidityKeccak(['uint256'], [s_int]))
    key_bytes = hashed_s_bytes[:16]
    key_hex = key_bytes.hex()
    key_uuid = '-'.join([
        key_hex[:8],
        key_hex[8:12],
        key_hex[12:16],
        key_hex[16:20],
        key_hex[20:],
    ])
    return key_uuid