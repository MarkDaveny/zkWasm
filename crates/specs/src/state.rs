use num_bigint::BigUint;
use serde::Deserialize;
use serde::Serialize;

#[cfg(feature = "continuation")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InitializationState<T, U> {
    pub eid: T,
    pub fid: T,
    pub iid: T,
    pub frame_id: T,
    pub sp: T,

    pub host_public_inputs: T,
    pub context_in_index: T,
    pub context_out_index: T,
    pub external_host_call_call_index: T,

    pub initial_memory_pages: T,
    pub maximal_memory_pages: T,

    pub jops: U,
}

#[cfg(not(feature = "continuation"))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InitializationState<T, U> {
    pub eid: T,
    pub fid: T,
    pub iid: T,
    pub frame_id: T,
    pub sp: T,

    pub host_public_inputs: T,
    pub context_in_index: T,
    pub context_out_index: T,
    pub external_host_call_call_index: T,

    pub initial_memory_pages: T,
    pub maximal_memory_pages: T,

    pub _phantom: std::marker::PhantomData<U>,
}

impl<T> InitializationState<T, T> {
    pub fn zip_for_each<U, E>(
        &self,
        other: &Self,
        mut closure: impl FnMut(&T, &T) -> Result<U, E>,
    ) -> Result<(), E> {
        closure(&self.eid, &other.eid)?;
        closure(&self.fid, &other.fid)?;
        closure(&self.iid, &other.iid)?;
        closure(&self.frame_id, &other.frame_id)?;
        closure(&self.sp, &other.sp)?;

        closure(&self.host_public_inputs, &other.host_public_inputs)?;
        closure(&self.context_in_index, &other.context_in_index)?;
        closure(&self.context_out_index, &other.context_out_index)?;
        closure(
            &self.external_host_call_call_index,
            &other.external_host_call_call_index,
        )?;

        closure(&self.initial_memory_pages, &other.initial_memory_pages)?;
        closure(&self.maximal_memory_pages, &other.maximal_memory_pages)?;

        #[cfg(feature = "continuation")]
        closure(&self.jops, &other.jops)?;

        Ok(())
    }
}

impl Default for InitializationState<u32, BigUint> {
    fn default() -> Self {
        Self {
            eid: Default::default(),
            fid: Default::default(),
            iid: Default::default(),
            frame_id: Default::default(),
            sp: Default::default(),

            host_public_inputs: Default::default(),
            context_in_index: Default::default(),
            context_out_index: Default::default(),
            external_host_call_call_index: Default::default(),

            initial_memory_pages: Default::default(),
            maximal_memory_pages: Default::default(),

            #[cfg(feature = "continuation")]
            jops: Default::default(),

            #[cfg(not(feature = "continuation"))]
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Clone> InitializationState<T, T> {
    pub fn plain(&self) -> Vec<T> {
        let mut v = vec![];

        v.push(self.eid.clone());
        v.push(self.fid.clone());
        v.push(self.iid.clone());
        v.push(self.frame_id.clone());
        v.push(self.sp.clone());

        v.push(self.host_public_inputs.clone());
        v.push(self.context_in_index.clone());
        v.push(self.context_out_index.clone());
        v.push(self.external_host_call_call_index.clone());

        v.push(self.initial_memory_pages.clone());
        v.push(self.maximal_memory_pages.clone());

        #[cfg(feature = "continuation")]
        v.push(self.jops.clone());

        v
    }
}

impl<T, U> InitializationState<T, U> {
    pub fn field_count() -> usize {
        if cfg!(feature = "continuation") {
            12
        } else {
            11
        }
    }

    pub fn for_each<V>(&self, f: impl FnMut(&T) -> V, g: impl FnMut(&U) -> V) {
        self.map(f, g);
    }

    pub fn map<V>(
        &self,
        mut f: impl FnMut(&T) -> V,
        mut _g: impl FnMut(&U) -> V,
    ) -> InitializationState<V, V> {
        InitializationState {
            eid: f(&self.eid),
            fid: f(&self.fid),
            iid: f(&self.iid),
            frame_id: f(&self.frame_id),
            sp: f(&self.sp),

            host_public_inputs: f(&self.host_public_inputs),
            context_in_index: f(&self.context_in_index),
            context_out_index: f(&self.context_out_index),
            external_host_call_call_index: f(&self.external_host_call_call_index),

            initial_memory_pages: f(&self.initial_memory_pages),
            maximal_memory_pages: f(&self.maximal_memory_pages),

            #[cfg(feature = "continuation")]
            jops: _g(&self.jops),

            #[cfg(not(feature = "continuation"))]
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T, U, E> InitializationState<Result<T, E>, Result<U, E>> {
    pub fn transpose(self) -> Result<InitializationState<T, U>, E> {
        Ok(InitializationState {
            eid: self.eid?,
            fid: self.fid?,
            iid: self.iid?,
            frame_id: self.frame_id?,
            sp: self.sp?,
            host_public_inputs: self.host_public_inputs?,
            context_in_index: self.context_in_index?,
            context_out_index: self.context_out_index?,
            external_host_call_call_index: self.external_host_call_call_index?,
            initial_memory_pages: self.initial_memory_pages?,
            maximal_memory_pages: self.maximal_memory_pages?,

            #[cfg(feature = "continuation")]
            jops: self.jops?,

            #[cfg(not(feature = "continuation"))]
            _phantom: std::marker::PhantomData,
        })
    }
}
