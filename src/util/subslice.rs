// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub trait SubsliceOffset {
    /// Get the byte offset of a given slice within its container.
    /// This assumes the given slice is not within the container
    /// (otherwise will `panic`).
    fn subslice_offset(&self, inner: &Self) -> usize;
}

impl<T> SubsliceOffset for [T] {
    fn subslice_offset(&self, inner: &Self) -> usize {
        let outer = self.as_ptr() as usize;
        let inner = inner.as_ptr() as usize;
        // Sanity check this makes sense
        assert!(inner >= outer && inner <= outer.wrapping_add(self.len()));
        // Calculate difference
        inner.wrapping_sub(outer)
    }
}
