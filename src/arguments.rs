use color_eyre::Result;
use itertools::Itertools;
use once_cell::sync::OnceCell;
use std::env;

use crate::SortingOption;

static INSTANCE: OnceCell<Arguments> = OnceCell::new();

#[derive(Debug)]
pub struct Arguments {
    pub sorting: Option<SortingOption>,
    pub all_media: bool,
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
            all_media: Self::read_all_media(&mut args),
        };

        INSTANCE
            .set(args)
            .expect("Arguments have already been initialized...");
        Ok(())
    }

    fn read_sort(args: &mut Vec<String>) -> Option<SortingOption> {
        for (i, arg) in args.iter_mut().enumerate() {
            if let Ok(sort) = SortingOption::from_str(&arg[1..]) {
                args.swap_remove(i);
                return Some(sort);
            }
        }

        None
    }

    fn read_all_media(args: &mut Vec<String>) -> bool {
        for (i, arg) in args.iter_mut().enumerate() {
            if arg == "-C" {
                args.swap_remove(i);
                return true;
            }
        }

        return false;
    }
}
