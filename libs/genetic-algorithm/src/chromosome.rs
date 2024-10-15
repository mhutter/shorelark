use std::ops::Index;

use crate::Number;

/// Encoding of the genetic information of an [`Individual`].
#[derive(Debug, Clone)]
pub struct Chromosome {
    genes: Vec<Number>,
}

impl Chromosome {
    #[must_use]
    pub fn len(&self) -> usize {
        self.genes.len()
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.genes.is_empty()
    }
    pub fn iter(&self) -> impl Iterator<Item = Number> + '_ {
        self.genes.iter().copied()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Number> {
        self.genes.iter_mut()
    }
}

impl Index<usize> for Chromosome {
    type Output = Number;

    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

impl FromIterator<Number> for Chromosome {
    fn from_iter<T: IntoIterator<Item = Number>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for Chromosome {
    type Item = Number;
    type IntoIter = std::vec::IntoIter<Number>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}

#[cfg(test)]
impl PartialEq for Chromosome {
    fn eq(&self, other: &Self) -> bool {
        approx::relative_eq!(self.genes.as_slice(), other.genes.as_slice())
    }
}

#[cfg(test)]
impl AsRef<[Number]> for Chromosome {
    fn as_ref(&self) -> &[Number] {
        &self.genes
    }
}
