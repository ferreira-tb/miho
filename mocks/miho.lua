miho.task = {
  foo = "cargo --version",
  bar = "rustc --version",
  task_a = "cmd /C echo task_a",
  task_b = "cmd /C echo task_b",
  task_c = "cmd /C echo task_c",
}

miho.task.outer = {
  task_d = "cmd /C echo outer task_d",
  task_e = "cmd /C echo outer task_e",
  task_f = "cmd /C echo outer task_f",
}

miho.task.outer.inner = {
  task_g = "cmd /C echo inner task_g",
  task_h = "cmd /C echo inner task_h",
  task_i = "cmd /C echo inner task_i",
  task_j = "cmd /C echo inner task_j",
  task_k = "cmd /C echo inner task_k",
  task_l = "cmd /C echo inner task_l",
  task_m = "cmd /C echo inner task_m",
  task_n = "cmd /C echo inner task_n",
}
