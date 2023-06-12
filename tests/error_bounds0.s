
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
  sub rsp, 32
  mov rax, 6
  mov [rsp+0], rax
  mov rbx, 2
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart0:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart0
  mov rax, 0
  mov [rsp+8], rax
  mov rax, 2
  mov [r15+16], rax
  mov rax, [rsp+8]
  mov [r15+8], rax
  mov rax, [rsp+0]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+0]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  mov rbx, rax
  and rbx, 3
  cmp rbx, 1
  jne throw_tag_check_error
  sub rax, 1
  mov r15, rax
  mov rax, 24
  cmp rax, [r15+0]
  jg throw_bounds_error
  cmp rax, 0
  jl throw_bounds_error
  sar rax, 1
  imul rax, 8
  mov rbx, rax
  mov qword rax, [r15+rbx]
  add rsp, 32
  ret
