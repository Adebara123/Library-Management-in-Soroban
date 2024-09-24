#![no_std]
use soroban_sdk::{contract, contractimpl,contracttype, symbol_short, vec, Env, Symbol, Vec};


#[derive(Clone, Debug)]
#[contracttype]
pub struct Book {
    title: Symbol,
    author: Symbol,
    year: u32,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct Library {
    books: Vec<Book>,
}

const LIBRARY_KEY: Symbol = symbol_short!("LIBRARY");

#[contract]
pub struct LibraryContract;

#[contractimpl]
impl LibraryContract {
    
    fn initialize(env: Env) {
       let library = Library {
        books: Vec::new(&env),
       };

       env.storage().instance().set(&LIBRARY_KEY, &library);
    }

    fn add_book(env: Env, title: Symbol, author: Symbol, year: u32) {
        
        let mut library: Library = env.storage().instance().get(&LIBRARY_KEY).unwrap_or_else(|| 
            Library {
                books: Vec::new(&env),
            }
        );

        library.books.push_back( Book {
            title,
            author, 
            year,
        });

        env.storage().instance().set(&LIBRARY_KEY, &library);
    }

    fn remove_book(env: Env, title: Symbol) {
        let mut library: Library = env.storage().instance().get(&LIBRARY_KEY).unwrap_or_else(||
            Library {
                books: Vec::new(&env),
            });

        let index = library.books.iter().position(|book| book.title == title);

        if let Some(i) = index {
            library.books.remove(i as u32);

            env.storage().instance().set(&LIBRARY_KEY, &library);
        }
        
    }

    fn find_book(env: Env, title: Symbol) -> Option<Book> {
        let library: Library = env.storage().instance().get(&LIBRARY_KEY).unwrap_or_else(||
            Library {
                books: Vec::new(&env),
            });

        library.books.iter().find(|book| book.title == title)

    }

    fn list_books(env: Env) -> Vec<Book> {
        let library: Library = env.storage().instance().get(&LIBRARY_KEY).unwrap_or_else(||
            Library {
                books: Vec::new(&env),
            });

            library.books
    }

    fn count_books(env: Env) -> u32 {
        let library: Library = env.storage().instance().get(&LIBRARY_KEY).unwrap_or_else(||
            Library {
                books: Vec::new(&env),
            });

        library.books.len() as u32
    }
}


pub trait LibraryTrait {
    fn initialize(env: Env);
    fn add_book(env: Env, title: Symbol, author: Symbol, year: u32);
    fn remove_book(env: Env, title: Symbol);
    fn find_book(env: Env, title: Symbol) -> Option<Book>;
    fn list_books(env: Env) -> Vec<Book>;
    fn count_books(env: Env) -> u32;
}




mod test;
