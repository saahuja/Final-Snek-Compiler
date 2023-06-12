
section .text
extern snek_error
extern snek_print
global our_code_starts_here
throw_overflow_error:
  mov rdi, 2
  push rsp
  call snek_error
  ret
throw_type_error:
  mov rdi, 1
  push rsp
  call snek_error
  ret
throw_bounds_error:
  mov rdi, 3
  push rsp
  call snek_error
  ret
throw_too_many_elements_error:
  mov rdi, 4
  push rsp
  call snek_error
  ret
throw_tag_check_error:
  mov rdi, 5
  push rsp
  call snek_error
  ret
throw_divide_by_0_error:
  mov rdi, 6
  push rsp
  call snek_error
  ret
throw_unequal_size_error:
  mov rdi, 7
  push rsp
  call snek_error
  ret

our_code_starts_here:
  mov r15, rsi
  sub rsp, 16
  mov rax, 10
  mov rbx, rax
  and rbx, 3
  cmp rbx, 1
  jne throw_tag_check_error
  sub rax, 1
  mov r15, rax
  mov rax, 8
  cmp rax, [r15+0]
  jg throw_bounds_error
  cmp rax, 0
  jl throw_bounds_error
  sar rax, 1
  imul rax, 8
  mov rbx, rax
  mov qword rax, [r15+rbx]
  add rsp, 16
  ret
