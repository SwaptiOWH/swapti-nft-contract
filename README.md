### Swapti_NFT

CONTRACT=dev-1678301484223-40643938770854

ACCOUNT=yairnava.testnet

Inicializar contrato:

    near call $CONTRACT new_default_meta '{"owner_id": "'$CONTRACT'"}' --accountId $CONTRACT

Crear un perfil:

    near call $CONTRACT create_profile '{"email": "yairnava@gmail.com","bio":"Esta es mi bio"}' --accountId $ACCOUNT --gas=300000000000000

Consultar perfil

    near view $CONTRACT get_profile '{"account_id": "yairnava.testnet"}'

Crear una solicitud

    near call $CONTRACT create_request '{"description": "Necesito ayuda para aprender JS"}' --accountId $ACCOUNT

Consultar todas las solicitudes

    near view $CONTRACT all_requests '{"from_index": "0", "limit": 50}'

Consultar solicitudes de un usuario

    near view $CONTRACT request_for_owner '{"account_id": "yairnava.testnet", "from_index": "0", "limit": 50}' 

Consultar solicitud por id

    near view $CONTRACT get_request '{"request_id": 0}'

Consultar a cuantas personas eh ayudado

    near view $CONTRACT get_number_swaps '{"accountid": "yairnava.testnet"}'

Atender solicitud

    near call $CONTRACT attend_request '{"request_id": 0}' --accountId darkyair.testnet

Finalizar solicitud

    near call $CONTRACT finish_request '{"request_id": 0}' --accountId yairnava.testnet --gas=300000000000000

Minar

    near call $CONTRACT mint_bronce --accountId yairnava.testnet --deposit 0.01 --gas=300000000000000

    near call $CONTRACT mint_plata --accountId yairnava.testnet --deposit 0.01 --gas=300000000000000

    near call $CONTRACT mint_oro --accountId yairnava.testnet --deposit 0.01 --gas=300000000000000

Consultar Insignias por usuario

    near view $CONTRACT nft_tokens_for_owner '{"account_id": "yairnava.testnet", "from_index": "0", "limit": 50}' 