#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;
	use sp_io::hashing::blake2_128;
	use scale_info::TypeInfo;
	use frame_support::traits::UnixTime;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type TimeProvider: UnixTime;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);
	
	#[derive(Encode, Decode, Default, Clone, PartialEq,TypeInfo)]
	#[scale_info(skip_type_params(T))]
	#[cfg_attr(feature = "std", derive(Debug))]
	pub struct WhispDetail<T: Config>{
		pub whash: [u8;16],
		pub whisper: T::AccountId,
		pub timestamp: u128,
		pub content: Vec<u8>,
	}


	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config>{
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		Whisp(T::AccountId,[u8;16],Vec<u8>,u128),
		GetWhisps(T::AccountId,Vec<[u8;16]>),
		GetWhispById(T::AccountId,[u8;16],Vec<u8>,u128)
		//GetEntities(T::AccountId, Vec<Vec<u8>>)
	}

	#[pallet::storage]
	#[pallet::getter(fn get_whisps)]
	pub type Whisps<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<[u8;16]>>;

	#[pallet::storage]
	#[pallet::getter(fn get_whisp)]
	pub type Whisp<T: Config> = StorageMap<_, Blake2_128Concat, [u8;16], WhispDetail<T>>;
	//pub type Role<T: Config> = StorageMap<_, Blake2_128Concat,  Vec<u8>, Vec<u8>, bool>;
	//pub type EntityRole<T: Config> = StorageMap<_, Blake2_128Concat,  Vec<u8>, Vec<u8>, bool>;
	//pub type Member<T: Config> = StorageMap<_, Blake2_128Concat,  T::AccountId, Vec<Vec<u8>>>;


	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn whisp(origin: OriginFor<T>, content: Vec<u8>) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;
			let now = T::TimeProvider::now().as_nanos();
			let now_vec = now.encode();
			let mut new_vec = [now_vec.clone(), content.clone()].concat();
			let hash =  blake2_128(&mut new_vec);
			let whsp = WhispDetail::<T>{
				whash: hash.clone(),
				whisper: caller.clone(),
				timestamp: now.clone(),
				content: content.clone(),
			};

			Whisp::<T>::insert(hash.clone(),whsp);

			let mut ws_list = vec![];
			//Append to whisp list
			let whps = Whisps::<T>::get(&caller);
			if whps.is_none(){
				ws_list.push(hash.clone());
				Whisps::<T>::insert(&caller,ws_list);
			}else{
				ws_list = whps.unwrap();
				ws_list.push(hash.clone());
				Whisps::<T>::insert(&caller,ws_list);
			}
			
			// emit event
			Self::deposit_event(Event::Whisp(caller,hash,content,now));
			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn get_own_whisps(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;
			let whisps = Whisps::<T>::get(&caller).unwrap();
			let mut w_l = vec![];
			for w in whisps{
				w_l.push(w);
			}
			Self::deposit_event(Event::GetWhisps(caller,w_l));
			Ok(().into())
		}


		#[pallet::weight(0)]
		pub fn get_whisp_by_id(origin: OriginFor<T>, hash: [u8;16]) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;
			let whisp = Whisp::<T>::get(hash.clone()).unwrap();

			Self::deposit_event(Event::GetWhispById(caller,
													hash.clone(),
													whisp.content,
													whisp.timestamp));
			Ok(().into())
		}

		
	}




}