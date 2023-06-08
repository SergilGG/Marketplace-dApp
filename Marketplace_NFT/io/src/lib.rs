#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};
use primitive_types::U256;

pub type ContractId = ActorId;
pub type TokenId = U256;
pub type Price = u128;
pub type TransactionId = u64;

pub struct MarketMetadata;

impl Metadata for MarketMetadata {
    type Init = In<InitMarket>;
    type Handle = InOut<MarketAction, MarketEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Market;
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct Market {
    pub admin_id: ActorId,
    pub treasury_id: ActorId,
    pub treasury_fee: u16,
    pub items: BTreeMap<(ContractId, TokenId), Item>,
    pub approved_nft_contracts: BTreeSet<ActorId>,
    pub approved_ft_contracts: BTreeSet<ActorId>,
    pub tx_id: TransactionId,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct ItemInfoArgs {
    nft_contract_id: ActorId,
    token_id: U256,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct InitMarket {
    pub admin_id: ActorId,
    pub treasury_id: ActorId,
    pub treasury_fee: u16,
}

#[derive(Debug, PartialEq, Eq, Default, Encode, Decode, TypeInfo, Clone)]
pub struct Auction {
    pub bid_period: u64,
    pub started_at: u64,
    pub ended_at: u64,
    pub current_price: Price,
    pub current_winner: ActorId,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone, PartialEq, Eq)]
pub enum MarketTx {
    Sale {
        buyer: ActorId,
    },
    Offer {
        ft_id: ContractId,
        price: Price,
        account: ActorId,
    },
    AcceptOffer,
    Withdraw {
        ft_id: ContractId,
        price: Price,
        account: ActorId,
    },
}

#[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo, Clone, Default)]
pub struct Item {
    pub token_id: TokenId,
    pub owner: ActorId,
    pub ft_contract_id: Option<ContractId>,
    pub price: Option<Price>,
    pub offers: BTreeMap<(Option<ContractId>, Price), ActorId>,
    pub tx: Option<(TransactionId, MarketTx)>,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum MarketAction {
    /// Añade direcciones de contratos NFT que se pueden listar en el mercado.
    ///
    /// # Requisitos:
    /// Sólo el administrador puede añadir cuentas NFT aprobadas.
    ///
    /// En caso de éxito responde [`MarketEvent::NftContractAdded`].
    AddNftContract(
        /// la dirección del contrato NFT
        ContractId,
    ),

    /// Añade las direcciones contractuales de las fichas fungibles con las que los usuarios pueden pagar las NFT.
    ///
    /// # Requisitos:
    /// Sólo el administrador puede añadir cuentas de fichas fungibles aprobadas.
    ///
    /// En caso de éxito responde [`MarketEvent::FtContractAdded`].
    AddFTContract(
        /// la dirección del contrato FT
        ContractId,
    ),

    /// Añade datos sobre el artículo del mercado.
    /// Si el artículo de ese NFT no existe en el mercado entonces se listará.
    /// Si el artículo existe entonces se utiliza esa acción para cambiar el precio o suspender la venta.
    ///
    /// # Requisitos
    /// * [`msg::source()`](gstd::msg::source) debe ser el propietario del NFT
    /// * `nft_contract_id` debe estar en la lista de `approved_nft_contracts`
    /// * si el elemento ya existe, no puede modificarse si hay una subasta activa
    ///
    /// En caso de éxito responde [`MarketEvent::MarketDataAdded`].
    AddMarketData {
        /// la dirección del contrato NFT
        nft_contract_id: ContractId,
        /// la dirección del contrato de token fungible (Si es `None` entonces el artículo se intercambia por el valor nativo)
        ft_contract_id: Option<ContractId>,
        /// el identificador NFT
        token_id: TokenId,
        /// el precio NFT (si es `None` entonces el artículo no está a la venta)
        price: Option<u128>,
    },

    /// Vende el NFT.
    ///
    /// # Requisitos:
    /// * El objeto NFT debe existir y estar a la venta.
    /// * Si el NFT se vende por un valor de Gear nativo, entonces un comprador debe tener un valor igual al precio.
    /// * Si el NFT se vende por tokens fungibles, el comprador debe tener suficientes tokens en el contrato de tokens fungibles.
    /// * No debe haber una subasta abierta sobre el artículo.
    ///
    /// En caso de éxito responde [`MarketEvent::ItemSold`].
    BuyItem {
        /// la dirección del contrato NFT
        nft_contract_id: ContractId,
        /// el ID del token
        token_id: TokenId,
    },

 
    /// Añade una oferta de precio al artículo.
    ///
    /// Requisitos:
    /// * El artículo NFT debe existir y estar listado en el mercado.
    /// * No debe haber una subasta en curso sobre el artículo.
    /// * Si un usuario hace una oferta en valor nativo Gear, entonces debe adjuntar un valor igual al precio indicado en los argumentos.
    /// * Si un usuario hace una oferta en tokens fungibles, entonces debe tener suficientes tokens en el contrato de tokens fungibles.
    /// * El precio no puede ser igual a 0.
    /// * No debe haber ofertas idénticas en el artículo.
    ///
    /// En caso de éxito responde [`MarketEvent::OfferAdded`].
    AddOffer {
        /// la dirección del contrato NFT
        nft_contract_id: ContractId,
        /// la dirección del contrato FT (si es 'None', la oferta se hace por el valor nativo)
        ft_contract_id: Option<ContractId>,
        /// El ID del NFT
        token_id: TokenId,
        /// el precio de oferta
        price: u128,
    },

    //// Retira fichas.
    ///
    /// Requisitos:
    /// * El artículo NFT debe existir y estar listado en el mercado.
    /// * Sólo el creador de la oferta puede retirar sus tokens.
    /// * La oferta con el hash indicado debe existir.
    ///
    /// En caso de éxito responde [`MarketEvent::Withdrawn`].
    Withdraw {
        /// la dirección del contrato NFT
        nft_contract_id: ContractId,
        /// la dirección del contrato FT (si es 'None', la oferta se hace por el valor nativo)
        ft_contract_id: Option<ContractId>,
        /// el id NFT
        token_id: TokenId,
        /// el precio ofertado (valor nativo)
        price: Price,
    },

    /// Acepta una oferta.
    ///
    /// Requisitos:
    /// * El artículo NFT debe existir y estar listado en el mercado.
    /// * Sólo el propietario puede aceptar una oferta.
    /// * No debe haber ninguna subasta en curso.
    /// * La oferta con el hash indicado debe existir.
    ///
    /// En caso de éxito responde [`MarketEvent::ItemSold`].
    AcceptOffer {
        /// la dirección del contrato NFT
        nft_contract_id: ContractId,
        /// el id de NFT
        token_id: TokenId,
        /// la dirección del contrato del token fungible
        ft_contract_id: Option<ContractId>,
        /// el precio de la oferta
        precio: Price,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum MarketEvent {
    NftContractAdded(ContractId),
    FtContractAdded(ContractId),
    MarketDataAdded {
        nft_contract_id: ContractId,
        token_id: TokenId,
        price: Option<u128>,
    },
    ItemSold {
        owner: ActorId,
        nft_contract_id: ContractId,
        token_id: TokenId,
    },
    NFTListed {
        nft_contract_id: ContractId,
        owner: ActorId,
        token_id: TokenId,
        price: Option<u128>,
    },
    OfferAdded {
        nft_contract_id: ContractId,
        ft_contract_id: Option<ActorId>,
        token_id: TokenId,
        price: u128,
    },
    OfferAccepted {
        nft_contract_id: ContractId,
        token_id: TokenId,
        new_owner: ActorId,
        price: u128,
    },
    Withdraw {
        nft_contract_id: ActorId,
        token_id: TokenId,
        price: u128,
    },
    TransactionFailed,
    RerunTransaction,
    TransferValue,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum MarketErr {
    NFTTransferFailed,
    TokenTransferFailed,
    WrongTransaction,
    RerunTransaction,
    WrongPrice,
    InvalidCaller,
    ItemOnAuction,
    ItemDoesNotExists,
    ItemIsNotOnSale,
    AuctionBidPeriodOrDurationIsInvalid,
    AuctionMinPriceIsZero,
    AuctionIsAlreadyExists,
    AuctionIsAlreadyEnded,
    AuctionIsNotOver,
    AuctionDoesNotExists,
    AuctionIsOpened,
    ContractNotApproved,
    OfferAlreadyExists,
    OfferShouldAcceptedByOwner,
    OfferIsNotExists,
}

pub fn all_items(state: <MarketMetadata as Metadata>::State) -> Vec<Item> {
    state.items.values().cloned().collect()
}

pub fn item_info(state: <MarketMetadata as Metadata>::State, args: &ItemInfoArgs) -> Option<Item> {
    state
        .items
        .get(&(args.nft_contract_id, args.token_id))
        .cloned()
}
