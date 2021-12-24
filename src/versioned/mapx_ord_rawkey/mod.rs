//!
//! Documents => [MapxRawVs](crate::versioned::mapx_raw)
//!

use crate::{
    common::{ende::ValueEnDe, BranchName, ParentBranchName, RawKey, VersionName},
    versioned::mapx_raw::{MapxRawVs, MapxRawVsIter},
    VsMgmt,
};
use ruc::*;
use serde::{Deserialize, Serialize};
use std::{marker::PhantomData, ops::RangeBounds};

/// Documents => [MapxRawVs](crate::versioned::mapx_raw::MapxRawVs)
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(bound = "")]
pub struct MapxOrdRawKeyVs<V>
where
    V: ValueEnDe,
{
    inner: MapxRawVs,
    p: PhantomData<V>,
}

impl<V> Default for MapxOrdRawKeyVs<V>
where
    V: ValueEnDe,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<V> MapxOrdRawKeyVs<V>
where
    V: ValueEnDe,
{
    #[inline(always)]
    pub fn new() -> Self {
        MapxOrdRawKeyVs {
            inner: MapxRawVs::new(),
            p: PhantomData,
        }
    }

    #[inline(always)]
    pub fn get(&self, key: &[u8]) -> Option<V> {
        self.inner
            .get(key)
            .map(|v| <V as ValueEnDe>::decode(&v).unwrap())
    }

    #[inline(always)]
    pub fn get_by_branch(&self, key: &[u8], branch_name: BranchName) -> Option<V> {
        self.inner
            .get_by_branch(key, branch_name)
            .map(|v| <V as ValueEnDe>::decode(&v).unwrap())
    }

    #[inline(always)]
    pub fn get_by_branch_version(
        &self,
        key: &[u8],
        branch_name: BranchName,
        version_name: VersionName,
    ) -> Option<V> {
        self.inner
            .get_by_branch_version(key, branch_name, version_name)
            .map(|v| <V as ValueEnDe>::decode(&v).unwrap())
    }

    #[inline(always)]
    pub fn get_le(&self, key: &[u8]) -> Option<(RawKey, V)> {
        self.inner
            .get_le(key)
            .map(|(k, v)| (k, <V as ValueEnDe>::decode(&v).unwrap()))
    }

    #[inline(always)]
    pub fn get_le_by_branch(
        &self,
        key: &[u8],
        branch_name: BranchName,
    ) -> Option<(RawKey, V)> {
        self.inner
            .get_le_by_branch(key, branch_name)
            .map(|(k, v)| (k, <V as ValueEnDe>::decode(&v).unwrap()))
    }

    #[inline(always)]
    pub fn get_le_by_branch_version(
        &self,
        key: &[u8],
        branch_name: BranchName,
        version_name: VersionName,
    ) -> Option<(RawKey, V)> {
        self.inner
            .get_le_by_branch_version(key, branch_name, version_name)
            .map(|(k, v)| (k, <V as ValueEnDe>::decode(&v).unwrap()))
    }

    #[inline(always)]
    pub fn get_ge(&self, key: &[u8]) -> Option<(RawKey, V)> {
        self.inner
            .get_ge(key)
            .map(|(k, v)| (k, <V as ValueEnDe>::decode(&v).unwrap()))
    }

    #[inline(always)]
    pub fn get_ge_by_branch(
        &self,
        key: &[u8],
        branch_name: BranchName,
    ) -> Option<(RawKey, V)> {
        self.inner
            .get_ge_by_branch(key, branch_name)
            .map(|(k, v)| (k, <V as ValueEnDe>::decode(&v).unwrap()))
    }

    #[inline(always)]
    pub fn get_ge_by_branch_version(
        &self,
        key: &[u8],
        branch_name: BranchName,
        version_name: VersionName,
    ) -> Option<(RawKey, V)> {
        self.inner
            .get_ge_by_branch_version(key, branch_name, version_name)
            .map(|(k, v)| (k, <V as ValueEnDe>::decode(&v).unwrap()))
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline(always)]
    pub fn len_by_branch(&self, branch_name: BranchName) -> usize {
        self.inner.len_by_branch(branch_name)
    }

    #[inline(always)]
    pub fn len_by_branch_version(
        &self,
        branch_name: BranchName,
        version_name: VersionName,
    ) -> usize {
        self.inner.len_by_branch_version(branch_name, version_name)
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline(always)]
    pub fn is_empty_by_branch(&self, branch_name: BranchName) -> bool {
        self.inner.is_empty_by_branch(branch_name)
    }

    #[inline(always)]
    pub fn is_empty_by_branch_version(
        &self,
        branch_name: BranchName,
        version_name: VersionName,
    ) -> bool {
        self.inner
            .is_empty_by_branch_version(branch_name, version_name)
    }

    #[inline(always)]
    pub fn insert(&self, key: RawKey, value: V) -> Result<Option<V>> {
        self.insert_ref(&key, &value)
    }

    #[inline(always)]
    pub fn insert_by_branch(
        &self,
        key: RawKey,
        value: V,
        branch_name: BranchName,
    ) -> Result<Option<V>> {
        self.insert_ref_by_branch(&key, &value, branch_name)
    }

    #[inline(always)]
    pub fn insert_ref(&self, key: &[u8], value: &V) -> Result<Option<V>> {
        self.inner
            .insert(key, &value.encode())
            .map(|v| v.map(|v| <V as ValueEnDe>::decode(&v).unwrap()))
    }

    #[inline(always)]
    pub fn insert_ref_by_branch(
        &self,
        key: &[u8],
        value: &V,
        branch_name: BranchName,
    ) -> Result<Option<V>> {
        self.inner
            .insert_by_branch(key, &value.encode(), branch_name)
            .map(|v| v.map(|v| <V as ValueEnDe>::decode(&v).unwrap()))
    }

    #[inline(always)]
    pub fn iter(&self) -> MapxOrdRawKeyVsIter<'_, V> {
        MapxOrdRawKeyVsIter {
            iter: self.inner.iter(),
            p: PhantomData,
        }
    }

    #[inline(always)]
    pub fn iter_by_branch(&self, branch_name: BranchName) -> MapxOrdRawKeyVsIter<'_, V> {
        MapxOrdRawKeyVsIter {
            iter: self.inner.iter_by_branch(branch_name),
            p: PhantomData,
        }
    }

    #[inline(always)]
    pub fn iter_by_branch_version(
        &self,
        branch_name: BranchName,
        version_name: VersionName,
    ) -> MapxOrdRawKeyVsIter<'_, V> {
        MapxOrdRawKeyVsIter {
            iter: self.inner.iter_by_branch_version(branch_name, version_name),
            p: PhantomData,
        }
    }

    #[inline(always)]
    pub fn range<'a, R: 'a + RangeBounds<RawKey>>(
        &'a self,
        bounds: R,
    ) -> MapxOrdRawKeyVsIter<'a, V> {
        MapxOrdRawKeyVsIter {
            iter: self.inner.range(bounds),
            p: PhantomData,
        }
    }

    #[inline(always)]
    pub fn range_by_branch<'a, R: 'a + RangeBounds<RawKey>>(
        &'a self,
        branch_name: BranchName,
        bounds: R,
    ) -> MapxOrdRawKeyVsIter<'a, V> {
        MapxOrdRawKeyVsIter {
            iter: self.inner.range_by_branch(branch_name, bounds),
            p: PhantomData,
        }
    }

    #[inline(always)]
    pub fn range_by_branch_version<'a, R: 'a + RangeBounds<RawKey>>(
        &'a self,
        branch_name: BranchName,
        version_name: VersionName,
        bounds: R,
    ) -> MapxOrdRawKeyVsIter<'a, V> {
        MapxOrdRawKeyVsIter {
            iter: self
                .inner
                .range_by_branch_version(branch_name, version_name, bounds),
            p: PhantomData,
        }
    }

    #[inline(always)]
    pub fn range_ref<'a, R: RangeBounds<&'a [u8]>>(
        &'a self,
        bounds: R,
    ) -> MapxOrdRawKeyVsIter<'a, V> {
        MapxOrdRawKeyVsIter {
            iter: self.inner.range_ref(bounds),
            p: PhantomData,
        }
    }

    #[inline(always)]
    pub fn range_ref_by_branch<'a, R: RangeBounds<&'a [u8]>>(
        &'a self,
        branch_name: BranchName,
        bounds: R,
    ) -> MapxOrdRawKeyVsIter<'a, V> {
        MapxOrdRawKeyVsIter {
            iter: self.inner.range_ref_by_branch(branch_name, bounds),
            p: PhantomData,
        }
    }

    #[inline(always)]
    pub fn range_ref_by_branch_version<'a, R: RangeBounds<&'a [u8]>>(
        &'a self,
        branch_name: BranchName,
        version_name: VersionName,
        bounds: R,
    ) -> MapxOrdRawKeyVsIter<'a, V> {
        MapxOrdRawKeyVsIter {
            iter: self.inner.range_ref_by_branch_version(
                branch_name,
                version_name,
                bounds,
            ),
            p: PhantomData,
        }
    }

    #[inline(always)]
    pub fn first(&self) -> Option<(RawKey, V)> {
        self.iter().next()
    }

    #[inline(always)]
    pub fn first_by_branch(&self, branch_name: BranchName) -> Option<(RawKey, V)> {
        self.iter_by_branch(branch_name).next()
    }

    #[inline(always)]
    pub fn first_by_branch_version(
        &self,
        branch_name: BranchName,
        version_name: VersionName,
    ) -> Option<(RawKey, V)> {
        self.iter_by_branch_version(branch_name, version_name)
            .next()
    }

    #[inline(always)]
    pub fn last(&self) -> Option<(RawKey, V)> {
        self.iter().next_back()
    }

    #[inline(always)]
    pub fn last_by_branch(&self, branch_name: BranchName) -> Option<(RawKey, V)> {
        self.iter_by_branch(branch_name).next_back()
    }

    #[inline(always)]
    pub fn last_by_branch_version(
        &self,
        branch_name: BranchName,
        version_name: VersionName,
    ) -> Option<(RawKey, V)> {
        self.iter_by_branch_version(branch_name, version_name)
            .next_back()
    }

    #[inline(always)]
    pub fn contains_key(&self, key: &[u8]) -> bool {
        self.inner.contains_key(key)
    }

    #[inline(always)]
    pub fn contains_key_by_branch(&self, key: &[u8], branch_name: BranchName) -> bool {
        self.inner.contains_key_by_branch(key, branch_name)
    }

    #[inline(always)]
    pub fn contains_key_by_branch_version(
        &self,
        key: &[u8],
        branch_name: BranchName,
        version_name: VersionName,
    ) -> bool {
        self.inner
            .contains_key_by_branch_version(key, branch_name, version_name)
    }

    #[inline(always)]
    pub fn remove(&self, key: &[u8]) -> Result<Option<V>> {
        self.inner
            .remove(key)
            .map(|v| v.map(|v| <V as ValueEnDe>::decode(&v).unwrap()))
    }

    #[inline(always)]
    pub fn remove_by_branch(
        &self,
        key: &[u8],
        branch_name: BranchName,
    ) -> Result<Option<V>> {
        self.inner
            .remove_by_branch(key, branch_name)
            .map(|v| v.map(|v| <V as ValueEnDe>::decode(&v).unwrap()))
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

impl<V> VsMgmt for MapxOrdRawKeyVs<V>
where
    V: ValueEnDe,
{
    crate::impl_vs_methods!();
}

pub struct MapxOrdRawKeyVsIter<'a, V>
where
    V: ValueEnDe,
{
    iter: MapxRawVsIter<'a>,
    p: PhantomData<V>,
}

impl<'a, V> Iterator for MapxOrdRawKeyVsIter<'a, V>
where
    V: ValueEnDe,
{
    type Item = (RawKey, V);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(k, v)| (k, <V as ValueEnDe>::decode(&v).unwrap()))
    }
}

impl<'a, V> DoubleEndedIterator for MapxOrdRawKeyVsIter<'a, V>
where
    V: ValueEnDe,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter
            .next_back()
            .map(|(k, v)| (k, <V as ValueEnDe>::decode(&v).unwrap()))
    }
}

impl<'a, V> ExactSizeIterator for MapxOrdRawKeyVsIter<'a, V> where V: ValueEnDe {}
