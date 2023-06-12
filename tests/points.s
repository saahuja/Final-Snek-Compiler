
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
addpoints:
sub rsp, 48
mov rax, 4
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
mov rax, [rsp+56]
mov rbx, rax
and rbx, 3
cmp rbx, 1
jne throw_tag_check_error
sub rax, 1
mov r15, rax
mov rax, 2
cmp rax, [r15+0]
jg throw_bounds_error
cmp rax, 0
jl throw_bounds_error
sar rax, 1
imul rax, 8
mov rbx, rax
mov qword rax, [r15+rbx]
mov [rsp+8], rax
mov rax, [rsp+64]
mov rbx, rax
and rbx, 3
cmp rbx, 1
jne throw_tag_check_error
sub rax, 1
mov r15, rax
mov rax, 2
cmp rax, [r15+0]
jg throw_bounds_error
cmp rax, 0
jl throw_bounds_error
sar rax, 1
imul rax, 8
mov rbx, rax
mov qword rax, [r15+rbx]
mov [rsp+16], rax
mov rbx, rax
or rbx, [rsp+8]
test rbx, 1
jne throw_type_error
add rax, [rsp+8]
jo throw_overflow_error
mov [rsp+8], rax
mov rax, [rsp+56]
mov rbx, rax
and rbx, 3
cmp rbx, 1
jne throw_tag_check_error
sub rax, 1
mov r15, rax
mov rax, 4
cmp rax, [r15+0]
jg throw_bounds_error
cmp rax, 0
jl throw_bounds_error
sar rax, 1
imul rax, 8
mov rbx, rax
mov qword rax, [r15+rbx]
mov [rsp+16], rax
mov rax, [rsp+64]
mov rbx, rax
and rbx, 3
cmp rbx, 1
jne throw_tag_check_error
sub rax, 1
mov r15, rax
mov rax, 4
cmp rax, [r15+0]
jg throw_bounds_error
cmp rax, 0
jl throw_bounds_error
sar rax, 1
imul rax, 8
mov rbx, rax
mov qword rax, [r15+rbx]
mov [rsp+24], rax
mov rbx, rax
or rbx, [rsp+16]
test rbx, 1
jne throw_type_error
add rax, [rsp+16]
jo throw_overflow_error
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
add rsp, 48
ret
our_code_starts_here:
  mov r15, rsi
  sub rsp, 32
  mov rax, 4
  mov [rsp+0], rax
  mov rbx, 2
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
  mov rax, 10
  mov [rsp+8], rax
  mov rax, 10
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
  mov [rsp+0], rax
  mov rax, 4
  mov [rsp+8], rax
  mov rbx, 2
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart2:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart2
  mov rax, 20
  mov [rsp+16], rax
  mov rax, 20
  mov [r15+16], rax
  mov rax, [rsp+16]
  mov [r15+8], rax
  mov rax, [rsp+8]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+8]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  sub rsp, 24
  mov rbx, [rsp+24]
  mov [rsp+0], rbx
  mov [rsp+8], rax
  mov [rsp+16], rdi
  call addpoints
  mov rdi, [rsp+16]
  add rsp, 24
  sub rsp, 8
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 8
  add rsp, 32
  ret
