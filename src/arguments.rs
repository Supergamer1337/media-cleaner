use color_eyre::Result;
use itertools::Itertools;
use once_cell::sync::OnceCell;
use std::env;

use crate::{Order, SortingOption};

static INSTANCE: OnceCell<Arguments> = OnceCell::new();

#[derive(Debug)]
pub struct Arguments {
    pub sorting: Option<SortingOption>,
}

impl Arguments {
    pub fn get_args() -> &'static Arguments {
        INSTANCE.get().expect("Arguments have not been initialised")
    }

    pub fn read_args() -> Result<()> {
        if let Some(_) = INSTANCE.get() {
            return Ok(());
        }

        let mut args = env::args().collect_vec();

        let args = Arguments {
            sorting: Self::read_sort(&mut args),
        };

        INSTANCE
            .set(args)
            .expect("Arguments have already been initialized...");
        Ok(())
    }

    fn read_sort(args: &mut Vec<String>) -> Option<SortingOption> {
        for (i, arg) in args.iter_mut().enumerate() {
            match arg.as_str() {
                "-n" => {
                    args.swap_remove(i);
                    return Some(SortingOption::Name(Order::Asc));
                }
                "-nd" => {
                    args.swap_remove(i);
                    return Some(SortingOption::Name(Order::Desc));
                }
                "-s" => {
                    args.swap_remove(i);
                    return Some(SortingOption::Size(Order::Desc));
                }
                "-sa" => {
                    args.swap_remove(i);
                    return Some(SortingOption::Size(Order::Asc));
                }
                _ => continue,
            }
        }

        None
    }
}
