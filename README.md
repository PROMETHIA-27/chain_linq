# Chain-LINQ

This crate is an implementation of .NET's LINQ query syntax in rust as a declarative macro. The declarative macro is generated by another declarative macro, specifically the
branching_parser from my other crate, big_mac. 

Available statements closely mirror standard LINQ operations; notably join is missing, as I was unable to find equivalent functionality from iterators.

Generally, each statement maps to an iterator method; here is a list of them, with a description and equivalent method if present:
- from # in #: selects and names variables from a collection. Maps to into_iter().
- select #: ends a linq query and returns an iterator. Maps to map().
- select # into #: partially ends a linq query and puts an iterator into a range variable. Creates a continuation.
- where #: selects only elements that match a particular criteria. Maps to filter().
- let # = #: creates a range element. Does not map to a method. 
- collect #: calls .collect().
- collect # as #: calls .collect with a generic type. Follows select syntax but the as parameter is the destination type.

**REQUIRES [ITERTOOLS](https://docs.rs/itertools/0.10.1/itertools/):**
- orderby #: sorts iterator ascending by a criteria. Maps to unstable_sort_by_key().
- orderby # ascending: sorts iterator ascending by a criteria. Maps to unstable_sort_by_key().
- orderby # descending: sorts iterator descending by a criteria. Maps to unstable_sort_by_key().rev().
- group # by #: groups elements into groups based on some criteria and returns the result. Maps to group_by(). 
- group # by # into #: groups elements into groups based on some criteria, and then creates a continuation.. Maps to group_by().

For more explanation of how LINQ works, check Microsoft's docs [here](https://docs.microsoft.com/en-us/dotnet/csharp/programming-guide/concepts/linq/)

Also useful: [Keyword breakdowns](https://docs.microsoft.com/en-us/dotnet/csharp/language-reference/keywords/query-keywords)

## Examples

```
use chain_linq::linq;

let xs = [(3, 1), (2, 2), (1, 3)];

let x = linq!(
    from (x, y) in xs
    let z = x + y
    select z into zs
    from z in zs
    select z * 2
);
```

```
use chain_linq::linq;

let xss = [vec!(27, 13, 12), vec!(69), vec!(76, 7, 420)];

let x = linq!(
    from xs in xss
    group xs by xs.len() into gs
    from iter in gs
    from x in iter
    collect x + 1 as Vec<i32>
);
```

```
use chain_linq::linq;

let xss = [vec!(27, 13, 12), vec!(69), vec!(76, 7, 420)];

let x = linq!(
    from xs in xss
    from x in xs.into_iter().rev()
    collect x as Vec<i32>
);
```