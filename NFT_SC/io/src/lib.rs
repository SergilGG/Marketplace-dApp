#![no_std]

use gear_lib::non_fungible_token::{
    io::{NFTApproval, NFTTransfer, NFTTransferPayout},
    royalties::*,
    state::NFTState,
    token::*,
};
use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};

pub use gear_lib::non_fungible_token::delegated::DelegatedApproveMessage;
use primitive_types::H256;

pub struct NFTMetadata;

impl Metadata for NFTMetadata {
    type Init = In<InitNFT>;
    type Handle = InOut<NFTAction, NFTEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = IoNFT;
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum NFTAction {

	//Mint se utiliza para crear nuevos tokens no fungibles y asignarlos a sus propietarios correspondientes.// 
    Mint {
        transaction_id: u64,
        token_metadata: TokenMetadata,
    },
    //Burn: La función Burn permite quemar (eliminar) un NFT existente. Recibe como parámetro el ID del token (token_id). Utiliza la función burn_token para eliminar el token no fungible, especificando el ID del token a quemar.//
    Burn {
        transaction_id: u64,
        token_id: TokenId,
    },
    //Transfer: La función Transfer se utiliza para transferir un NFT de un propietario a otro. Recibe como parámetros el ID del token (token_id) y el ID del nuevo propietario (receiver_id). Utiliza la función transfer_token para transferir el token no fungible, especificando el ID del token y el ID del nuevo propietario.//
    Transfer {
        transaction_id: u64,
        to: ActorId,
        token_id: TokenId,
    },
    //TransferPayout: Esta función TransferPayout se utiliza para transferir un NFT de un propietario a otro y realizar un pago adicional. Recibe como parámetros el ID del token (token_id), el ID del nuevo propietario (receiver_id) y el monto a pagar (payout). Utiliza la función transfer_token_with_payout para transferir el token no fungible y realizar el pago adicional, pasando los parámetros correspondientes.//
    TransferPayout {
        transaction_id: u64,
        to: ActorId,
        token_id: TokenId,
        amount: u128,
    },
    
    //NFTPayout: La función NFTPayout se utiliza para realizar un pago adicional asociado a un NFT existente. Recibe como parámetros el ID del token (token_id) y el monto a pagar (payout). Utiliza la función add_payout_to_token para agregar el pago adicional al token no fungible especificado.//
    NFTPayout {
        owner: ActorId,
        amount: u128,
    },
    
    //Approve: La función Approve se utiliza para otorgar a otro usuario la capacidad de transferir un NFT en nombre del propietario actual. Recibe como parámetros el ID del token (token_id) y el ID del aprobado (approved_id). Utiliza la función approve_transfer para otorgar el permiso de transferencia, pasando el ID del token y el ID del usuario aprobado.//
    Approve {
        transaction_id: u64,
        to: ActorId,
        token_id: TokenId,
    },
    
    //DelegatedApprove: Esta función DelegatedApprove se utiliza para otorgar a otro usuario la capacidad de transferir un NFT en nombre del propietario actual de forma delegada. Recibe como parámetros el ID del token (token_id), el ID del propietario actual (current_owner_id) y el ID del aprobado (approved_id). Utiliza la función delegated_approve_transfer para otorgar el permiso de transferencia delegada, pasando los parámetros correspondientes.//
    DelegatedApprove {
        transaction_id: u64,
        message: DelegatedApproveMessage,
        signature: [u8; 64],
    },
    
    //Owner: La función Owner se utiliza para obtener el propietario actual de un NFT. Recibe como parámetro el ID del token (token_id). Utiliza la función get_token_owner para obtener el ID del propietario del token no fungible especificado.//
    Owner {
        token_id: TokenId,
    },
    
    //IsApproved: Esta función IsApproved se utiliza para verificar si un usuario está aprobado para transferir un NFT específico. Recibe como parámetros el ID del token (token_id) y el ID del usuario (user_id). Utiliza la función is_approved_for_token para comprobar si el usuario está aprobado para transferir el token no fungible especificado.//
    IsApproved {
        to: ActorId,
        token_id: TokenId,
    },
    
    //Clear: La función Clear se utiliza para borrar todos los datos relacionados con un NFT específico. Recibe como parámetro el ID del token (token_id). Utiliza la función clear_token_data para eliminar todos los datos asociados al token no fungible especificado.//
    Clear {
        transaction_hash: H256,
    },
}


//VARIABLES
//transaction_id: Esta variable representa un identificador único de transacción. En el código, se utiliza en varios lugares para identificar de manera única una transacción relacionada con la acción realizada en el contrato. Cada vez que se realiza una acción, como la creación de un nuevo token, la transferencia de un token o la aprobación de un token, se asigna un transaction_id a esa transacción específica. Este identificador se utiliza para rastrear y asociar eventos y resultados relacionados con esa transacción en particular.//

//token_metadata: Esta variable representa los metadatos asociados a un token específico. En Rust, los metadatos se representan mediante la estructura TokenMetadata. Estos metadatos proporcionan información adicional y descriptiva sobre el token, como el nombre, el símbolo, la descripción, las imágenes o cualquier otro detalle relevante. Los metadatos pueden variar para cada instancia de token y se utilizan para describir y caracterizar de manera única cada token no fungible.//

//token_id: Esta variable representa el identificador único de un token no fungible (NFT). Cada NFT tiene asignado un token_id único que lo distingue de otros tokens. En los códigos, se utiliza para identificar de manera única un token específico en las operaciones de minting (creación), transferencia y otras acciones relacionadas con los NFTs. El token_id permite realizar un seguimiento de la propiedad y el estado de los tokens individuales.//

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitNFT {
    pub name: String,
    pub symbol: String,
    pub base_uri: String,
    pub royalties: Option<Royalties>,
}

#[derive(Encode, Decode, TypeInfo, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum NFTEvent {
    Transfer(NFTTransfer),
    TransferPayout(NFTTransferPayout),
    NFTPayout(Payout),
    Approval(NFTApproval),
    Owner {
        owner: ActorId,
        token_id: TokenId,
    },
    IsApproved {
        to: ActorId,
        token_id: TokenId,
        approved: bool,
    },
}

#[derive(Debug, Clone, Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct IoNFTState {
    pub name: String,
    pub symbol: String,
    pub base_uri: String,
    pub owner_by_id: Vec<(TokenId, ActorId)>,
    pub token_approvals: Vec<(TokenId, Vec<ActorId>)>,
    pub token_metadata_by_id: Vec<(TokenId, Option<TokenMetadata>)>,
    pub tokens_for_owner: Vec<(ActorId, Vec<TokenId>)>,
    pub royalties: Option<Royalties>,
}

#[derive(Debug, Clone, Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct IoNFT {
    pub token: IoNFTState,
    pub token_id: TokenId,
    pub owner: ActorId,
    pub transactions: Vec<(H256, NFTEvent)>,
}

impl From<&NFTState> for IoNFTState {
    fn from(value: &NFTState) -> Self {
        let NFTState {
            name,
            symbol,
            base_uri,
            owner_by_id,
            token_approvals,
            token_metadata_by_id,
            tokens_for_owner,
            royalties,
        } = value;

        let owner_by_id = owner_by_id
            .iter()
            .map(|(hash, actor_id)| (*hash, *actor_id))
            .collect();

        let token_approvals = token_approvals
            .iter()
            .map(|(key, approvals)| (*key, approvals.iter().copied().collect()))
            .collect();

        let token_metadata_by_id = token_metadata_by_id
            .iter()
            .map(|(id, metadata)| (*id, metadata.clone()))
            .collect();

        let tokens_for_owner = tokens_for_owner
            .iter()
            .map(|(id, tokens)| (*id, tokens.clone()))
            .collect();

        Self {
            name: name.clone(),
            symbol: symbol.clone(),
            base_uri: base_uri.clone(),
            owner_by_id,
            token_approvals,
            token_metadata_by_id,
            tokens_for_owner,
            royalties: royalties.clone(),
        }
    }
}
