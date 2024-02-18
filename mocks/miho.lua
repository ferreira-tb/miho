miho.task = {
  cargo = "cargo --version",
  rustc = "rustc --version",
  task_a = "cmd /C echo task_a",
  task_b = "cmd /C echo task_b",
  task_c = "cmd /C echo task_c",
}

miho.task.inner = {
  task_d = "cmd /C echo inner task_d",
  task_e = "cmd /C echo inner task_e",
  task_f = "cmd /C echo inner task_f",
}

miho.task.inner.mock = {
  task_g = "cmd /C echo mock task_g",
  task_h = "cmd /C echo mock task_h",
  task_i = "cmd /C echo mock task_i",
  task_j = "cmd /C echo mock task_j",
  task_k = "cmd /C echo mock task_k",
  task_l = "cmd /C echo mock task_l",
  task_m = "cmd /C echo mock task_m",
  task_n = "cmd /C echo mock task_n",
}
