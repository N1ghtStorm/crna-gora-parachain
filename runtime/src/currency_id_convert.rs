use super::{
	AccountId, Balances, Call, Event, Origin, ParachainInfo, ParachainSystem, PolkadotXcm, Runtime,
	WeightToFee, XcmpQueue,
};
use crate::sp_api_hidden_includes_construct_runtime::hidden_include::traits::Get;
use crate::CurrencyId;
use common::primitives::{ParachainAssets, SoraNativeAssets};
use core::marker::PhantomData;
use frame_support::{
	log, match_types, parameter_types,
	traits::{Everything, Nothing},
	weights::Weight,
};
use orml_traits::{location::AbsoluteReserveProvider, parameter_type_with_key, MultiCurrency};
use orml_xcm_support::{
	DepositToAlternative, IsNativeConcrete, MultiCurrencyAdapter, MultiNativeAsset,
};
use pallet_xcm::XcmPassthrough;
use polkadot_parachain::primitives::Sibling;
use polkadot_runtime_common::impls::ToAuthor;
use sp_std::vec::Vec;
use xcm::latest::prelude::*;
use xcm_builder::{
	AccountId32Aliases, AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom, CurrencyAdapter,
	EnsureXcmOrigin, FixedWeightBounds, IsConcrete, LocationInverter, NativeAsset, ParentIsPreset,
	RelayChainAsNative, SiblingParachainAsNative, SiblingParachainConvertsVia,
	SignedAccountId32AsNative, SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit,
	UsingComponents,
};
use xcm_executor::{traits::ShouldExecute, XcmExecutor};

pub const SORA_PARA_ID: u32 = 2001;
pub const XOR_KEY: &[u8] = b"XOR";
pub const PSWAP_KEY: &[u8] = b"PSWAP";
pub const VAL_KEY: &[u8] = b"VAL";
pub const XSTUSD_KEY: &[u8] = b"XSTUSD";
// pub const XOR_KEY: &[u8] = &hex_literal::hex!("0020");
// pub const PSWAP_KEY: &[u8] = &hex_literal::hex!("0030");
// pub const VAL_KEY: &[u8] = &hex_literal::hex!("0040");
// pub const XSTUSD_KEY: &[u8] = &hex_literal::hex!("0050");

pub fn get_para_key_multilocation(para_id: u32, key: Vec<u8>) -> Option<MultiLocation> {
	Some(MultiLocation::new(1, X2(Parachain(para_id), GeneralKey(key.to_vec()))))
}

pub struct CurrencyIdConvert;

impl sp_runtime::traits::Convert<CurrencyId, Option<MultiLocation>> for CurrencyIdConvert {
	fn convert(id: CurrencyId) -> Option<MultiLocation> {
		match id {
			CurrencyId::KSM => Some(Parent.into()),
			CurrencyId::SoraNative(sora_asset) => match sora_asset {
				SoraNativeAssets::XOR => get_para_key_multilocation(SORA_PARA_ID, XOR_KEY.to_vec()),
				SoraNativeAssets::PSWAP => {
					get_para_key_multilocation(SORA_PARA_ID, PSWAP_KEY.to_vec())
				},
				SoraNativeAssets::VAL => get_para_key_multilocation(SORA_PARA_ID, VAL_KEY.to_vec()),
				SoraNativeAssets::XSTUSD => {
					get_para_key_multilocation(SORA_PARA_ID, XSTUSD_KEY.to_vec())
				},
			},
			CurrencyId::Parachain(parachain_asset) => match parachain_asset {
				ParachainAssets::KAR => None,
				_ => None,
			},
			_ => None,
		}
	}
}

impl sp_runtime::traits::Convert<MultiLocation, Option<CurrencyId>> for CurrencyIdConvert {
	fn convert(multilocation: MultiLocation) -> Option<CurrencyId> {
		log::trace!(
			target: "xcm",
			"================== MultiLocation : {:?} =====================",
			multilocation
		);
		if multilocation == MultiLocation::parent() {
			return Some(CurrencyId::KSM);
		}
		match multilocation {
			MultiLocation { parents, interior } if parents == 1 => match interior {
				X2(Parachain(SORA_PARA_ID), GeneralKey(key)) => match key {
					key if key == XOR_KEY => Some(CurrencyId::SoraNative(SoraNativeAssets::XOR)),
					key if key == PSWAP_KEY => {
						Some(CurrencyId::SoraNative(SoraNativeAssets::PSWAP))
					},
					key if key == VAL_KEY => Some(CurrencyId::SoraNative(SoraNativeAssets::VAL)),
					key if key == XSTUSD_KEY => {
						Some(CurrencyId::SoraNative(SoraNativeAssets::XSTUSD))
					},
					_ => None,
				},
				_ => None,
			},
			MultiLocation { parents, interior } if parents == 0 => match interior {
				X1(GeneralKey(k)) if k == XOR_KEY => Some(CurrencyId::SoraNative(SoraNativeAssets::XOR)),
				_ => None,
			},
			_ => None,
		}
	}
}

impl sp_runtime::traits::Convert<MultiAsset, Option<CurrencyId>> for CurrencyIdConvert {
	fn convert(a: MultiAsset) -> Option<CurrencyId> {
		log::trace!(
			target: "xcm",
			"============ MultiAsset : {:?} ====================",
			a
		);
		if let MultiAsset { fun: Fungible(_), id: Concrete(id) } = a {
			Self::convert(id)
		} else {
			Option::None
		}
	}
}
