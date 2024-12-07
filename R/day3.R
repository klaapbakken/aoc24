library(dplyr)

mul <- \(x, y) x * y 

input <- here::here("inputs", "day3", "day3.txt") |> 
  readr::read_lines() |> 
  paste0(collapse="")

mul_res <- input |> 
  stringr::str_extract_all("(mul\\(\\d*,\\d*\\))") |> 
  purrr::flatten_chr() |> 
  purrr::map_dbl(\(x) eval(parse(text=x))) 

sum(mul_res)

muls <- stringr::str_locate_all(input, "(mul\\(\\d*,\\d*\\))")
dos <- stringr::str_locate_all(input, "do\\(\\)") 
donts <- stringr::str_locate_all(input, "don't\\(\\)")

muls |> 
  dplyr::bind_cols(res = mul_res) |> 
  mutate(valid = NA) |> 
  bind_rows(
    dos[[1]] |> 
      as_tibble() |> 
      mutate(
        res = NA, 
        valid = TRUE
      )
    ) |>
  bind_rows(
    donts[[1]] |> 
      as_tibble() |> 
      mutate(
        res = NA, 
        valid = FALSE
      )
    ) |> 
  bind_rows(
    tibble::as_tibble_row(list(start=-1L, end=-1L, res=NA, valid=TRUE))
  ) |> 
  arrange(start) |> 
  tidyr::fill(valid) |> 
  summarise(
    sum = sum(res * valid, na.rm = TRUE)
  )
