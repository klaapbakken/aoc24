library(dplyr)

input <- here::here("inputs", "day1", "day1.txt") |> 
  readr::read_lines() |> 
  tibble::as_tibble_col("lines") |> 
  tidyr::separate_wider_delim(lines, "   ", names = c("left", "right")) |> 
  mutate(across(everything(), as.integer))

input |> 
  mutate(
    left = sort(left),
    right = sort(right),
    diff = abs(left - right)
  ) |> 
  pull(diff) |> 
  sum()

input |> 
  group_by(left) |> 
  (\(x) {
    x |> 
    mutate(
      num_equals = sum(cur_group()$left == x$right)
    )  
  })() |> 
  ungroup() |> 
  summarise(sum(left * num_equals))


