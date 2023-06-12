
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
  mov [rsp+0], rax
  mov rbx, 0
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
  mov rax, [rsp+0]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+0]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  mov [rsp+0], rax
  mov rax, 10
  mov [rsp+8], rax
  mov rbx, 0
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart1:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart1
  mov rax, [rsp+8]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+8]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  mov [rsp+8], rax
  mov rbx, rax
  xor rbx, [rsp+0]
  and rbx, 3
  cmp rbx, 0
  jne throw_type_error
  mov rax, [rsp+8]
  and rax, 3
  cmp rax, 1
  jne throw_type_error
  mov rax, [rsp+0]
  and rax, 3
  cmp rax, 1
  jne throw_type_error
  mov rax, [rsp+8]
  mov rbx, [rsp+0]
  sub rax, 1
  sub rbx, 1
  mov rax, [rax+0]
  mov rbx, [rbx+0]
  cmp rax, rbx
  jne break_loop3
  mov rcx, rax
  imul rcx, 8
  mov rax, [rsp+0]
  mov rbx, [rsp+8]
  mov rdx, 7
  arr_loopstart2:
  mov rax, [rsp+0]
  mov rbx, [rsp+8]
  sub rax, 1
  sub rbx, 1
  add rax, rcx
  add rbx, rcx
  mov rax, [rax+0]
  mov rbx, [rbx+0]
  cmp rax, rbx
  jne break_loop3
  sub rcx, 8
  cmp rcx, 0
  jnz arr_loopstart2
  jz end_loop4
  break_loop3:
  mov rdx, 3
  end_loop4:
  mov rax, rdx
  jo throw_overflow_error
  sub rsp, 8
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 8
  add rsp, 16
  ret
