use std::hint::black_box;
use rand::Rng;

use criterion::Criterion;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub trait IAnimal : Send + Sync
{
    fn nb_os_cache(&self) -> u32 { 0 }
    fn nb_vie(&self) -> u32 { 0 }
}

pub enum Animal
{
    Chien(Chien),
    Chat(Chat),
}
impl IAnimal for Animal
{
    fn nb_os_cache(&self) -> u32 {
        match self
        {
            Animal::Chien(chien) => chien.nb_os_cache(),
            Animal::Chat(chat) => chat.nb_os_cache(),
        }
    }

    fn nb_vie(&self) -> u32 {
        match self
        {
            Animal::Chien(chien) => chien.nb_vie(),
            Animal::Chat(chat) => chat.nb_vie(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Chien { nb_os_cache : u32 }
impl IAnimal for Chien 
{
    fn nb_os_cache(&self) -> u32 { self.nb_os_cache }
}

#[derive(Debug, Clone, Copy)]
pub struct Chat { nb_vie : u32 }
impl IAnimal for Chat 
{
    fn nb_vie(&self) -> u32 { self.nb_vie }
}
fn main() 
{
    let capacity = 200_000_000;

    println!("init the animal list of {} animals...", capacity);

    let mut animals_enum : Vec<Animal> = Vec::with_capacity(capacity);
    let mut animals_box_dyn : Vec<Box<dyn IAnimal>> = Vec::with_capacity(capacity);

    let mut rng = rand::rng();
    
    for _ in 0..capacity
    {
        if rng.random()
        {
            let chien = Chien{ nb_os_cache : rng.random::<u32>() % 50 };
            animals_enum.push(Animal::Chien(chien));
            animals_box_dyn.push(Box::new(chien));
        }else
        {
            let chat = Chat{ nb_vie : rng.random::<u32>() % 10 };
            animals_enum.push(Animal::Chat(chat));
            animals_box_dyn.push(Box::new(chat));
        }
    }

    println!("done !");

    let mut criterion: Criterion = Criterion::default().measurement_time(std::time::Duration::new(20, 0)).nresamples(100).sample_size(10);
    
    criterion.bench_function("iter enum sum os", |b| b.iter(|| 
        {
            let nb_os : u32 = animals_enum.iter().map(|v| v.nb_os_cache()).sum();
            black_box(nb_os); // Prevent compiler optimization

            //println!("nb os enum    : {}", nb_os);
        }
    ));
    criterion.bench_function("iter dyn box sum os", |b| b.iter(|| 
        {
            let nb_os : u32 = animals_box_dyn.iter().map(|v| v.nb_os_cache()).sum();
            black_box(nb_os);

            //println!("nb os dyn box : {}", nb_os);
        }
    ));

    criterion.bench_function("iter enum sum os par", |b| b.iter(|| 
        {
            let nb_os : u32 = animals_enum.par_iter().map(|v| v.nb_os_cache()).sum();
            black_box(nb_os);

            //println!("nb os enum par   : {}", nb_os);
        }
    ));
    criterion.bench_function("iter dyn box sum os par", |b| b.iter(|| 
        {
            let nb_os : u32 = animals_box_dyn.par_iter().map(|v| v.nb_os_cache()).sum();
            black_box(nb_os);

            //println!("nb os dyn box par : {}", nb_os);
        }
    ));

    println!("Done");
}
