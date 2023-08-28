enum LibraryItem {
    Book {
        title: String,
        author: String,
        pages: u32,
    },
    Magazine {
        title: String,
        issue: u32,
    },
    CD {
        title: String,
        artist: String,
        tracks: u32,
    },
}

impl LibraryItem {
    fn display(&self) {
        match self {
            LibraryItem::Book {
                title,
                author,
                pages,
            } => {
                println!(
                    "Book - Title: {}, Author: {}, Pages: {}",
                    title, author, pages
                );
            }
            LibraryItem::Magazine { title, issue } => {
                println!("Magazine - Title: {}, Issue: {}", title, issue);
            }
            LibraryItem::CD {
                title,
                artist,
                tracks,
            } => {
                println!(
                    "CD - Title: {}, Artist: {}, Tracks: {}",
                    title, artist, tracks
                );
            }
        }
    }
}

fn main() {
    let library_items = vec![
        LibraryItem::Book {
            title: String::from("The Great Gatsby"),
            author: String::from("F. Scott Fitzgerald"),
            pages: 180,
        },
        LibraryItem::Magazine {
            title: String::from("National Geographic"),
            issue: 202,
        },
        LibraryItem::CD {
            title: String::from("Thriller"),
            artist: String::from("Michael Jackson"),
            tracks: 9,
        },
    ];

    for item in library_items {
        item.display();
    }
}
