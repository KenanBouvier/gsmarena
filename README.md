# GSMArena
An Api interface for GSMArena device specifications.

- Obtains the spec sheet listed on their database.
- Returns data in json format or as an object for further use.


The core functions take in one argument: the "id", which can be found in the URL as such:

```
https://www.gsmarena.com/samsung_galaxy_a54-12070.php -> samsung_galaxy_a54-12070 
```


## Example

```rust
fn main() {
    let json_format = gsmarena::get_specification_json("samsung_galaxy_a54-12070");
    println!("{}", response);

    let object_format = gsmarena::get_specification("samsung_galaxy_a54-12070");
    // println!("{:#?}", object)
}
```

### Output format (Truncated)
```json
{
  "name": "samsung_galaxy_a54-12070",
  "specification": [
    {
      "category_title": "Network",
      "category_spec": [
        [
          "Technology",
          "GSM / HSPA / LTE / 5G"
        ],
        [
          "2G bands",
          "GSM 850 / 900 / 1800 / 1900 - SIM 1 & SIM 2 (dual-SIM model only)"
        ],
      ]
    },
    {
      "category_title": "Body",
      "category_spec": [
        [
          "Dimensions",
          "158.2 x 76.7 x 8.2 mm (6.23 x 3.02 x 0.32 in)"
        ],
        [
          "Weight",
          "202 g (7.13 oz)"
        ],
      ]
    }
  ]
}
```
