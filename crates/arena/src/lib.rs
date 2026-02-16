
use std::collections::HashSet;

type Index = u32;
type Gen = u32;
pub struct Slot<T>{
    generation: Gen,
    value: Option<T>
}

pub struct Arena<T> {
    slots: Vec<Slot<T>>,
    free: Vec<Index>
}

pub struct Id {
    pub index: Index, 
    pub generation: Gen
}

impl<T> Arena<T> {
    pub fn new() -> Arena<T> {
        return Arena {slots: Vec::<Slot<T>>::new(), free: Vec::<Index>::new()}
    }


    pub fn alloc(self: &mut Self, value: T) -> Id {
        if let Some(idx_u32) = self.free.pop(){
            let idx  = idx_u32 as usize;
            debug_assert!(idx < self.slots.len());
            debug_assert!(self.slots[idx].value.is_none());
            self.slots[idx].value = Some(value);
            return Id {index: idx_u32, generation: self.slots[idx].generation};
        }else{
            let id: Id = Id {index: self.slots.len() as Index, generation: 0 as Gen};
            let slot = Slot {value: Some(value), generation: 0 as Gen};
            self.slots.push(slot);
            return id;
        }
    }

    pub fn get(self: &Self, id: &Id) -> Option<&T> {
        if id.index < self.slots.len() as Index {
            let idx = id.index as usize;
            if id.generation == self.slots[idx].generation{
                return self.slots[idx].value.as_ref();
            }else{
                return None;
            }
        }else{
            return None;
        }
    }

    pub fn get_mut(self: &mut Self, id: &Id) -> Option<&mut T> {
        if id.index < self.slots.len() as Index{
            let idx = id.index as usize;
            if id.generation == self.slots[idx].generation{
                return self.slots[idx].value.as_mut();
            }else{
                return None;
            }
        }else{
            return None;
        }
    }    

    pub fn remove(self: &mut Self, id: &Id) -> Option<T> {
        if id.index  < self.slots.len() as Index{
            let idx = id.index as usize;
            if id.generation == self.slots[idx].generation{
                let value =  self.slots[idx].value.take();
                self.slots[idx].generation = self.slots[idx].generation + 1;
                self.free.push(id.index);
                return value;
            }else{
                return None
            }
        }else{
            return None
        }
    }

    pub fn debug_check(self: &Self) -> bool {
        // Check if every index in free is in bounds
        let out_bound_counts = self.free.iter().filter(|&&index| (index as usize) >= self.slots.len()).count();
        if out_bound_counts > 0{
            return false;
        }
        let non_none_freed_values_count = self.free.iter().filter(|&&index| self.slots[index as usize].value.is_some()).count();
        if non_none_freed_values_count > 0 {
            return false;
        }
        let mut seen = HashSet::<Index>::new();
        for &i in self.free.iter(){
            if !seen.insert(i){
                return false;
            }
        }

        let live_count = self.slots.iter().filter(|s| s.value.is_some()).count();
        let free_count = self.slots.iter().filter(|s| s.value.is_none()).count();
        if live_count + free_count != self.slots.len(){
            return false;
        }
        if free_count != self.free.len(){
            return false;
        }
        return true;
    }
}