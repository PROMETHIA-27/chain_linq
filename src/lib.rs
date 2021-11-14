#![feature(trace_macros, log_syntax)]
#![allow(unused_macros, unused_imports)]
#![recursion_limit = "256"]

mod macros {
    #[cfg(feature = "expanded")]
    #[macro_export]
    macro_rules! linq_impl {
        // Archetype: {name of working variable} {cumulative previous expression} {statements to add to closures as prefixes} {pattern to match, remaining tokens}
        // Short names: {var} {prev} {prefix} {___, toks}

        // From clauses
        (from {$var:pat} in {$base:expr}, $($toks:tt)*) => ({
            $crate::linq_impl!{ 
                {$var}
                {$base.into_iter()}
                {}
                {$($toks)*}
            } 
        });

        (
            {$var:pat}
            {$prev:expr}
            {$($prefix:stmt)*}
            {from {$new_var:pat} in {$new_base:expr}, $($toks:tt)*}
        ) => {
            $crate::linq_impl!{
                {$new_var}
                {$prev.flat_map(|$var| { $($prefix)* $new_base.into_iter() })}
                {}
                {$($toks)*}
            }
        };

        // Where clause
        (
            {$var:pat}
            {$prev:expr}
            {$($prefix:stmt)*}
            {where {$cond:expr}, $($toks:tt)*}
        ) => {
            $crate::linq_impl!{
                {$var}
                {$prev.filter(|$var| { $($prefix)* $cond })}
                {}
                {$($toks)*}
            }
        };

        // Select clauses
        (
            {$var:pat}
            {$prev:expr}
            {$($prefix:stmt)*}
            {select {$result:expr} into {$new_base:ident}, $($toks:tt)*}
        ) => {
            $crate::linq_impl!{
                {$new_base}
                {$prev.map(|$var| { $($prefix)* $result })}
                {}
                {$($toks)*}
            }
        };

        (
            {$var:pat}
            {$prev:expr}
            {$($prefix:stmt)*}
            {select {$result:expr},}
        ) => {
            $prev.map(|$var| { $($prefix)* $result })
        };

        // Orderby clauses


        // (
        //     {$var:pat}
        //     {$prev:expr}
        //     {$($prefix:stmt)*}
        //     {orderby {$prop:expr}, $($toks:tt)*}
        // ) => {
        //     linq_impl!{
        //         {$var}
        //         {$prev.filter(|$var| { $($prefix)* $cond })}
        //         {}
        //         {$($toks)*}
        //     }
        // };

        // ($x:pat => $prev:expr ; { $($prefix:stmt)* }; orderby ($($target:expr)+) ascending , $($tree:tt)*) => {
        //     linq_impl!{ $x => { $prev.sorted_unstable_by_key(|$x| { $($prefix)* $($target)+ }) }; {} ; $($tree)* }
        // };

        // ($x:pat => $prev:expr ; { $($prefix:stmt)* }; orderby ($($target:expr)+) descending , $($tree:tt)*) => {
        //     linq_impl!{ $x => { $prev.sorted_unstable_by_key(|$x| { $($prefix)*  $($target)+ }).rev() } ; {} ; $($tree)* }
        // };

        // ($x:pat => $prev:expr ; { $($prefix:stmt)* }; orderby ($($target:expr)*) , $($tree:tt)*) => {
        //     linq_impl!{ $x => $prev ; { $($prefix)* } ; orderby ($($target)*) ascending , $($tree)* }
        // };

        // ($x:pat => $prev:expr ; { $($prefix:stmt)* }; orderby $($target:expr)* , $($tree:tt)*) => {
        //     linq_impl!{ $x => $prev ; { $($prefix)* } ; orderby ($($target)*) ascending , $($tree)* }
        // };

        // Group clauses
        (
            {$var:pat}
            {$prev:expr}
            {$($prefix:stmt)*}
            {group {$grouped:ident} by {$grouper:expr} into {$group:ident}, $($toks:tt)*}
        ) => {
            {
                use ::itertools::*;
                
                $crate::linq_impl!{
                    {$group}
                    {$prev.group_by(|$grouped| { $($prefix)* $grouper }).into_iter().map(|g| g.1)}
                    {}
                    {$($toks)*}
                }
            }
            
        };

        // ($x:pat => $prev:expr ; { $($prefix:stmt)* }; group ($grouped:pat) by ($($grouper:expr)+) into $group:ident , $($tree:tt)*) => {
        //     linq!{ $group => $prev.group_by(|$grouped| { $($prefix)* $($grouper)+ }).into_iter().map(|g| g.1) ; {} ; $($tree)* }
        // };

        (
            {$var:pat}
            {$prev:expr}
            {$($prefix:stmt)*}
            {group {$grouped:ident} by {$grouper:expr}, $($toks:tt)*}
        ) => {
            {
                use ::itertools::*;
                
                $prev.group_by(|$grouped| { $($prefix)* $grouper })
            }
        };

        // ($x:pat => $prev:expr ; { $($prefix:stmt)* }; group ($grouped:pat) by $($grouper:expr)+ ) => {
        //     $prev.group_by(|$grouped| { $($prefix)* $($grouper)+ })
        // };

        // Let clause
        (
            {$var:pat}
            {$prev:expr}
            {$($prefix:stmt)*}
            {let {$new_var:pat} = {$query:expr}, $($toks:tt)*}
        ) => {
            $crate::linq_impl!{
                {$var}
                {$prev}
                {$($prefix)* let $new_var = $query }
                {$($toks)*}
            }
        };

        // Collect clause
        (
            {$var:pat}
            {$prev:expr}
            {$($prefix:stmt)*}
            {collect {$result:expr},}
        ) => {
            $prev.map(|$var| $result).collect()
        };

        (
            {$var:pat}
            {$prev:expr}
            {$($prefix:stmt)*}
            {collect {$result:expr} as {$collection:ty},}
        ) => {
            $prev.map(|$var| $result).collect::<$collection>()
        };

        (
            {$var:pat}
            {$prev:expr}
            {$($prefix:stmt)*}
            {}
        ) => {
            compile_error!("Unfinished linq query!");
        };
    }



    // #[cfg(not(feature = "expanded"))]
    // macro_rules! linq {
    //     (from $x:pat in $($list:expr)+ , $($tree:tt)*) => ({
    //         linq!{ $x => $($list)+.into_iter() ; {} ; $($tree)* } 
    //     });

    //     ($x:pat => $prev:expr ; { $($prefix:stmt)* }; from $newx:pat in $list:ident , $($tree:tt)*) => {
    //         linq!{ $newx => $prev.flat_map(|$list| { $($prefix)* $list.into_iter() }) ; {} ; $($tree)* }
    //     };

    //     ($x:pat => $prev:expr ; { $($prefix:stmt)* }; where $($cond:expr)+ , $($tree:tt)*) => {
    //         linq!{ $x => $prev.filter(|$x| { $($prefix)* $($cond)+ }) ; {} ;$($tree)* }
    //     };

    //     ($x:pat => $prev:expr ; { $($prefix:stmt)* }; select ($($result:expr)+) into $new:ident , $($tree:tt)*) => {
    //         linq!{ $new => $prev.map(|$x| { $($prefix)* $($result)+ }) ; {} ; $($tree)* }
    //     };

    //     ($x:pat => $prev:expr ; { $($prefix:stmt)* }; select $($result:expr)+ ) => {
    //         $prev.map(|$x| { $($prefix)* $($result)+ })
    //     };

    //     ($x:pat => $prev:expr ; { $($prefix:stmt)* }; orderby ($($target:expr)+) ascending , $($tree:tt)*) => {
    //         linq!{ $x => { $prev.sorted_unstable_by_key(|$x| { $($prefix)* $($target)+ }) }; {} ; $($tree)* }
    //     };

    //     ($x:pat => $prev:expr ; { $($prefix:stmt)* }; orderby ($($target:expr)+) descending , $($tree:tt)*) => {
    //         linq!{ $x => { $prev.sorted_unstable_by_key(|$x| { $($prefix)*  $($target)+ }).rev() } ; {} ; $($tree)* }
    //     };

    //     ($x:pat => $prev:expr ; { $($prefix:stmt)* }; orderby ($($target:expr)*) , $($tree:tt)*) => {
    //         linq!{ $x => $prev ; { $($prefix)* } ; orderby ($($target)*) ascending , $($tree)* }
    //     };

    //     ($x:pat => $prev:expr ; { $($prefix:stmt)* }; orderby $($target:expr)* , $($tree:tt)*) => {
    //         linq!{ $x => $prev ; { $($prefix)* } ; orderby ($($target)*) ascending , $($tree)* }
    //     };

    //     ($x:pat => $prev:expr ; { $($prefix:stmt)* }; group ($grouped:pat) by ($($grouper:expr)+) into $group:ident , $($tree:tt)*) => {
    //         linq!{ $group => $prev.group_by(|$grouped| { $($prefix)* $($grouper)+ }).into_iter().map(|g| g.1) ; {} ; $($tree)* }
    //     };

    //     ($x:pat => $prev:expr ; { $($prefix:stmt)* }; group ($grouped:pat) by $($grouper:expr)+ ) => {
    //         $prev.group_by(|$grouped| { $($prefix)* $($grouper)+ })
    //     };

    //     ($x:pat => $prev:expr ; { $($prefix:stmt)* }; let $newx:pat = $($query:expr)+ , $($tree:tt)*) => {
    //         linq!($x => $prev; { $($prefix)* let $newx = $($query)+ }; $($tree)*)
    //     };

    //     ($x:pat => $prev:expr ; $($prefix:stmt)* ; ) => {
    //         compile_error!("Unfinished linq query!");
    //     };

    //     (from $x:pat in $list:ident $($tree:tt)*) => {
    //         compile_error!("You must comma separate linq query steps!");
    //     };

    //     (from $x:pat in $list:ident $(,)?) => {
    //         compile_error!("You must follow a from expression with more expressions");
    //     };
    // }

    // Patterns:
    // from # in #
    // where #
    // select #
    // select # into #
    // orderby #
    // orderby # ascending/descending
    // group # by #
    // group # by # into #
    // let # = #

    // trace_macros!(true);
    big_mac::branching_parser!(
        @unroll
        chain_linq;
        linq
        linq_parser
        linq_filter
        chain_linq::linq_impl;
        {
            from
            {
                #
                {
                    in
                    {
                        # {}
                    }
                }
            }
        }
        {
            where
            {
                # {}
            }
        }
        {
            select 
            {
                #
                {
                    into 
                    {
                        # {}
                    }
                }
                {}
            }
        }
        {
            orderby
            {
                # 
                {}
                {
                    ascending
                }
                {
                    descending
                }
            }
        }
        {
            group
            {
                #
                {
                    by
                    {
                        #
                        {}
                        {
                            into 
                            {
                                # {}
                            }
                        }
                    }
                }
            }
        }
        {
            let
            {
                #
                {
                    =
                    {
                        # {}
                    }
                }
            }
        }
        {
            collect
            {
                #
                {}
                {
                    as
                    {
                        # {}
                    }
                }
            }
        }
    );

    pub(crate) use {linq, linq_parser, linq_filter, linq_impl};
}