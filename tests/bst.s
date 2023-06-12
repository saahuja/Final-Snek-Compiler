
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
length:
sub rsp, 32
mov rax, [rsp+40]
mov [rsp+0], rax
xor rax, rax
add rax, 1
mov [rsp+8], rax
mov rbx, rax
xor rbx, [rsp+0]
test rbx, 1
jne throw_type_error
cmp [rsp+0], rax
mov rbx, 7
mov rax, 3
cmove rax, rbx
jo throw_overflow_error
cmp rax, 3
je ifelse1
mov rax, 0
jmp ifend0
ifelse1:
mov rax, [rsp+40]
mov rbx, rax
and rbx, 3
cmp rbx, 1
jne throw_tag_check_error
sub rax, 1
mov r15, rax
mov rax, 0
cmp rax, [r15+0]
jg throw_bounds_error
cmp rax, 0
jl throw_bounds_error
sar rax, 1
imul rax, 8
mov rbx, rax
mov qword rax, [r15+rbx]
ifend0:
add rsp, 32
ret
insert_bst:
sub rsp, 64
mov rax, [rsp+72]
sub rsp, 16
mov [rsp+0], rax
mov [rsp+8], rdi
call length
mov rdi, [rsp+8]
add rsp, 16
mov [rsp+0], rax
mov rax, 0
mov [rsp+8], rax
mov rbx, rax
xor rbx, [rsp+0]
test rbx, 1
jne throw_type_error
cmp [rsp+0], rax
mov rbx, 7
mov rax, 3
cmove rax, rbx
jo throw_overflow_error
cmp rax, 3
je ifelse3
mov rax, 6
mov [rsp+0], rax
mov rbx, 3
imul rbx, 2
cmp rbx, rax
jg throw_too_many_elements_error
imul rax, 8
mov rbx, rax
sar rbx, 1
arr_loopstart4:
mov qword [r15+rbx], 1
sub rbx, 8
jnz arr_loopstart4
mov rax, [rsp+80]
mov [rsp+8], rax
xor rax, rax
add rax, 1
mov [rsp+16], rax
xor rax, rax
add rax, 1
mov [r15+24], rax
mov rax, [rsp+8]
mov [r15+8], rax
mov rax, [rsp+16]
mov [r15+16], rax
mov rax, [rsp+0]
mov [r15+0], rax
mov rax, r15
add rax, 1
mov rbx, [rsp+0]
add rbx, 1
imul rbx, 8
add r15, rbx
jmp ifend2
ifelse3:
mov rax, [rsp+72]
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
mov [rsp+0], rax
mov rax, [rsp+72]
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
mov [rsp+8], rax
mov rax, [rsp+72]
mov rbx, rax
and rbx, 3
cmp rbx, 1
jne throw_tag_check_error
sub rax, 1
mov r15, rax
mov rax, 6
cmp rax, [r15+0]
jg throw_bounds_error
cmp rax, 0
jl throw_bounds_error
sar rax, 1
imul rax, 8
mov rbx, rax
mov qword rax, [r15+rbx]
mov [rsp+16], rax
mov rax, [rsp+80]
mov [rsp+24], rax
mov rax, [rsp+0]
mov [rsp+32], rax
mov rbx, rax
or rbx, [rsp+24]
test rbx, 1
jne throw_type_error
cmp [rsp+24], rax
mov rbx, 7
mov rax, 3
cmovl rax, rbx
jo throw_overflow_error
cmp rax, 3
je ifelse6
mov rax, 6
mov [rsp+24], rax
mov rbx, 3
imul rbx, 2
cmp rbx, rax
jg throw_too_many_elements_error
imul rax, 8
mov rbx, rax
sar rbx, 1
arr_loopstart7:
mov qword [r15+rbx], 1
sub rbx, 8
jnz arr_loopstart7
mov rax, [rsp+0]
mov [rsp+32], rax
mov rax, [rsp+8]
mov [rsp+40], rax
mov rax, [rsp+80]
sub rsp, 24
mov rbx, [rsp+64]
mov [rsp+0], rbx
mov [rsp+8], rax
mov [rsp+16], rdi
call insert_bst
mov rdi, [rsp+16]
add rsp, 24
mov [rsp+40], rax
mov rax, [rsp+16]
mov [r15+24], rax
mov rax, [rsp+32]
mov [r15+8], rax
mov rax, [rsp+40]
mov [r15+16], rax
mov rax, [rsp+24]
mov [r15+0], rax
mov rax, r15
add rax, 1
mov rbx, [rsp+24]
add rbx, 1
imul rbx, 8
add r15, rbx
jmp ifend5
ifelse6:
mov rax, 6
mov [rsp+24], rax
mov rbx, 3
imul rbx, 2
cmp rbx, rax
jg throw_too_many_elements_error
imul rax, 8
mov rbx, rax
sar rbx, 1
arr_loopstart8:
mov qword [r15+rbx], 1
sub rbx, 8
jnz arr_loopstart8
mov rax, [rsp+0]
mov [rsp+32], rax
mov rax, [rsp+8]
mov [rsp+40], rax
mov rax, [rsp+16]
mov [rsp+48], rax
mov rax, [rsp+80]
sub rsp, 24
mov rbx, [rsp+72]
mov [rsp+0], rbx
mov [rsp+8], rax
mov [rsp+16], rdi
call insert_bst
mov rdi, [rsp+16]
add rsp, 24
mov [r15+24], rax
mov rax, [rsp+32]
mov [r15+8], rax
mov rax, [rsp+40]
mov [r15+16], rax
mov rax, [rsp+24]
mov [r15+0], rax
mov rax, r15
add rax, 1
mov rbx, [rsp+24]
add rbx, 1
imul rbx, 8
add r15, rbx
ifend5:
ifend2:
add rsp, 64
ret
lookup:
sub rsp, 64
mov rax, [rsp+72]
sub rsp, 16
mov [rsp+0], rax
mov [rsp+8], rdi
call length
mov rdi, [rsp+8]
add rsp, 16
mov [rsp+0], rax
mov rax, 0
mov [rsp+8], rax
mov rbx, rax
xor rbx, [rsp+0]
test rbx, 1
jne throw_type_error
cmp [rsp+0], rax
mov rbx, 7
mov rax, 3
cmove rax, rbx
jo throw_overflow_error
cmp rax, 3
je ifelse10
mov rax, 3
jmp ifend9
ifelse10:
mov rax, [rsp+72]
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
mov [rsp+0], rax
mov rax, [rsp+72]
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
mov [rsp+8], rax
mov rax, [rsp+72]
mov rbx, rax
and rbx, 3
cmp rbx, 1
jne throw_tag_check_error
sub rax, 1
mov r15, rax
mov rax, 6
cmp rax, [r15+0]
jg throw_bounds_error
cmp rax, 0
jl throw_bounds_error
sar rax, 1
imul rax, 8
mov rbx, rax
mov qword rax, [r15+rbx]
mov [rsp+16], rax
mov rax, [rsp+80]
mov [rsp+24], rax
mov rax, [rsp+0]
mov [rsp+32], rax
mov rbx, rax
xor rbx, [rsp+24]
test rbx, 1
jne throw_type_error
cmp [rsp+24], rax
mov rbx, 7
mov rax, 3
cmove rax, rbx
jo throw_overflow_error
cmp rax, 3
je ifelse12
mov rax, 7
jmp ifend11
ifelse12:
mov rax, [rsp+80]
mov [rsp+24], rax
mov rax, [rsp+0]
mov [rsp+32], rax
mov rbx, rax
or rbx, [rsp+24]
test rbx, 1
jne throw_type_error
cmp [rsp+24], rax
mov rbx, 7
mov rax, 3
cmovl rax, rbx
jo throw_overflow_error
cmp rax, 3
je ifelse14
mov rax, [rsp+8]
mov [rsp+24], rax
mov rax, [rsp+80]
sub rsp, 24
mov rbx, [rsp+48]
mov [rsp+0], rbx
mov [rsp+8], rax
mov [rsp+16], rdi
call lookup
mov rdi, [rsp+16]
add rsp, 24
jmp ifend13
ifelse14:
mov rax, [rsp+16]
mov [rsp+24], rax
mov rax, [rsp+80]
sub rsp, 24
mov rbx, [rsp+48]
mov [rsp+0], rbx
mov [rsp+8], rax
mov [rsp+16], rdi
call lookup
mov rdi, [rsp+16]
add rsp, 24
ifend13:
ifend11:
ifend9:
add rsp, 64
ret
our_code_starts_here:
  mov r15, rsi
  sub rsp, 64
  mov rax, 6
  mov [rsp+0], rax
  mov rbx, 3
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart15:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart15
  mov rax, 6
  mov [rsp+8], rax
  mov rax, 6
  mov [rsp+16], rax
  mov rbx, 3
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart16:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart16
  mov rax, 4
  mov [rsp+24], rax
  xor rax, rax
  add rax, 1
  mov [rsp+32], rax
  xor rax, rax
  add rax, 1
  mov [r15+24], rax
  mov rax, [rsp+24]
  mov [r15+8], rax
  mov rax, [rsp+32]
  mov [r15+16], rax
  mov rax, [rsp+16]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+16]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  mov [rsp+16], rax
  mov rax, 6
  mov [rsp+24], rax
  mov rbx, 3
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart17:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart17
  mov rax, 10
  mov [rsp+32], rax
  xor rax, rax
  add rax, 1
  mov [rsp+40], rax
  xor rax, rax
  add rax, 1
  mov [r15+24], rax
  mov rax, [rsp+32]
  mov [r15+8], rax
  mov rax, [rsp+40]
  mov [r15+16], rax
  mov rax, [rsp+24]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+24]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  mov [r15+24], rax
  mov rax, [rsp+8]
  mov [r15+8], rax
  mov rax, [rsp+16]
  mov [r15+16], rax
  mov rax, [rsp+0]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+0]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  mov [rsp+0], rax
  mov rax, 12
  sub rsp, 24
  mov rbx, [rsp+24]
  mov [rsp+0], rbx
  mov [rsp+8], rax
  mov [rsp+16], rdi
  call lookup
  mov rdi, [rsp+16]
  add rsp, 24
  sub rsp, 8
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 8
  mov rax, 6
  mov [rsp+0], rax
  mov rbx, 3
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart18:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart18
  mov rax, 6
  mov [rsp+8], rax
  mov rax, 6
  mov [rsp+16], rax
  mov rbx, 3
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart19:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart19
  mov rax, 4
  mov [rsp+24], rax
  xor rax, rax
  add rax, 1
  mov [rsp+32], rax
  xor rax, rax
  add rax, 1
  mov [r15+24], rax
  mov rax, [rsp+24]
  mov [r15+8], rax
  mov rax, [rsp+32]
  mov [r15+16], rax
  mov rax, [rsp+16]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+16]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  mov [rsp+16], rax
  mov rax, 6
  mov [rsp+24], rax
  mov rbx, 3
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart20:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart20
  mov rax, 10
  mov [rsp+32], rax
  xor rax, rax
  add rax, 1
  mov [rsp+40], rax
  xor rax, rax
  add rax, 1
  mov [r15+24], rax
  mov rax, [rsp+32]
  mov [r15+8], rax
  mov rax, [rsp+40]
  mov [r15+16], rax
  mov rax, [rsp+24]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+24]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  mov [r15+24], rax
  mov rax, [rsp+8]
  mov [r15+8], rax
  mov rax, [rsp+16]
  mov [r15+16], rax
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
  sub rsp, 24
  mov rbx, [rsp+24]
  mov [rsp+0], rbx
  mov [rsp+8], rax
  mov [rsp+16], rdi
  call lookup
  mov rdi, [rsp+16]
  add rsp, 24
  sub rsp, 8
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 8
  mov rax, 6
  mov [rsp+0], rax
  mov rbx, 3
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart21:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart21
  mov rax, 6
  mov [rsp+8], rax
  mov rax, 6
  mov [rsp+16], rax
  mov rbx, 3
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart22:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart22
  mov rax, 4
  mov [rsp+24], rax
  xor rax, rax
  add rax, 1
  mov [rsp+32], rax
  xor rax, rax
  add rax, 1
  mov [r15+24], rax
  mov rax, [rsp+24]
  mov [r15+8], rax
  mov rax, [rsp+32]
  mov [r15+16], rax
  mov rax, [rsp+16]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+16]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  mov [rsp+16], rax
  mov rax, 6
  mov [rsp+24], rax
  mov rbx, 3
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart23:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart23
  mov rax, 10
  mov [rsp+32], rax
  xor rax, rax
  add rax, 1
  mov [rsp+40], rax
  xor rax, rax
  add rax, 1
  mov [r15+24], rax
  mov rax, [rsp+32]
  mov [r15+8], rax
  mov rax, [rsp+40]
  mov [r15+16], rax
  mov rax, [rsp+24]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+24]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  mov [r15+24], rax
  mov rax, [rsp+8]
  mov [r15+8], rax
  mov rax, [rsp+16]
  mov [r15+16], rax
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
  sub rsp, 24
  mov rbx, [rsp+24]
  mov [rsp+0], rbx
  mov [rsp+8], rax
  mov [rsp+16], rdi
  call lookup
  mov rdi, [rsp+16]
  add rsp, 24
  sub rsp, 8
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 8
  mov rax, 6
  mov [rsp+0], rax
  mov rbx, 3
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart24:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart24
  mov rax, 6
  mov [rsp+8], rax
  mov rax, 6
  mov [rsp+16], rax
  mov rbx, 3
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart25:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart25
  mov rax, 4
  mov [rsp+24], rax
  xor rax, rax
  add rax, 1
  mov [rsp+32], rax
  xor rax, rax
  add rax, 1
  mov [r15+24], rax
  mov rax, [rsp+24]
  mov [r15+8], rax
  mov rax, [rsp+32]
  mov [r15+16], rax
  mov rax, [rsp+16]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+16]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  mov [rsp+16], rax
  mov rax, 6
  mov [rsp+24], rax
  mov rbx, 3
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart26:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart26
  mov rax, 10
  mov [rsp+32], rax
  xor rax, rax
  add rax, 1
  mov [rsp+40], rax
  xor rax, rax
  add rax, 1
  mov [r15+24], rax
  mov rax, [rsp+32]
  mov [r15+8], rax
  mov rax, [rsp+40]
  mov [r15+16], rax
  mov rax, [rsp+24]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+24]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  mov [r15+24], rax
  mov rax, [rsp+8]
  mov [r15+8], rax
  mov rax, [rsp+16]
  mov [r15+16], rax
  mov rax, [rsp+0]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+0]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  mov [rsp+0], rax
  mov rax, 6
  sub rsp, 24
  mov rbx, [rsp+24]
  mov [rsp+0], rbx
  mov [rsp+8], rax
  mov [rsp+16], rdi
  call lookup
  mov rdi, [rsp+16]
  add rsp, 24
  sub rsp, 8
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 8
  xor rax, rax
  add rax, 1
  mov [rsp+0], rax
  mov rax, [rsp+0]
  mov [rsp+8], rax
  mov rax, 4
  sub rsp, 24
  mov rbx, [rsp+32]
  mov [rsp+0], rbx
  mov [rsp+8], rax
  mov [rsp+16], rdi
  call insert_bst
  mov rdi, [rsp+16]
  add rsp, 24
  mov [rsp+0], rax
  mov rax, [rsp+0]
  mov [rsp+8], rax
  mov rax, 6
  sub rsp, 24
  mov rbx, [rsp+32]
  mov [rsp+0], rbx
  mov [rsp+8], rax
  mov [rsp+16], rdi
  call insert_bst
  mov rdi, [rsp+16]
  add rsp, 24
  mov [rsp+0], rax
  mov rax, [rsp+0]
  mov [rsp+8], rax
  mov rax, 10
  sub rsp, 24
  mov rbx, [rsp+32]
  mov [rsp+0], rbx
  mov [rsp+8], rax
  mov [rsp+16], rdi
  call insert_bst
  mov rdi, [rsp+16]
  add rsp, 24
  mov [rsp+0], rax
  mov rax, [rsp+0]
  mov [rsp+8], rax
  mov rax, 486
  sub rsp, 24
  mov rbx, [rsp+32]
  mov [rsp+0], rbx
  mov [rsp+8], rax
  mov [rsp+16], rdi
  call insert_bst
  mov rdi, [rsp+16]
  add rsp, 24
  mov [rsp+0], rax
  mov rax, [rsp+0]
  mov [rsp+8], rax
  mov rax, 2000
  sub rsp, 24
  mov rbx, [rsp+32]
  mov [rsp+0], rbx
  mov [rsp+8], rax
  mov [rsp+16], rdi
  call insert_bst
  mov rdi, [rsp+16]
  add rsp, 24
  mov [rsp+0], rax
  mov rax, [rsp+0]
  mov [rsp+8], rax
  mov rax, 24
  sub rsp, 24
  mov rbx, [rsp+32]
  mov [rsp+0], rbx
  mov [rsp+8], rax
  mov [rsp+16], rdi
  call insert_bst
  mov rdi, [rsp+16]
  add rsp, 24
  mov [rsp+0], rax
  mov rax, [rsp+0]
  mov [rsp+8], rax
  mov rax, 40
  sub rsp, 24
  mov rbx, [rsp+32]
  mov [rsp+0], rbx
  mov [rsp+8], rax
  mov [rsp+16], rdi
  call insert_bst
  mov rdi, [rsp+16]
  add rsp, 24
  mov [rsp+0], rax
  mov rax, [rsp+0]
  mov [rsp+8], rax
  mov rax, 60
  sub rsp, 24
  mov rbx, [rsp+32]
  mov [rsp+0], rbx
  mov [rsp+8], rax
  mov [rsp+16], rdi
  call insert_bst
  mov rdi, [rsp+16]
  add rsp, 24
  mov [rsp+0], rax
  mov rax, [rsp+0]
  mov [rsp+8], rax
  mov rax, 12
  sub rsp, 24
  mov rbx, [rsp+32]
  mov [rsp+0], rbx
  mov [rsp+8], rax
  mov [rsp+16], rdi
  call insert_bst
  mov rdi, [rsp+16]
  add rsp, 24
  mov [rsp+0], rax
  mov rax, [rsp+0]
  mov [rsp+8], rax
  mov rax, 4
  sub rsp, 24
  mov rbx, [rsp+32]
  mov [rsp+0], rbx
  mov [rsp+8], rax
  mov [rsp+16], rdi
  call lookup
  mov rdi, [rsp+16]
  add rsp, 24
  sub rsp, 24
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 24
  mov rax, [rsp+0]
  mov [rsp+8], rax
  mov rax, 6
  sub rsp, 24
  mov rbx, [rsp+32]
  mov [rsp+0], rbx
  mov [rsp+8], rax
  mov [rsp+16], rdi
  call lookup
  mov rdi, [rsp+16]
  add rsp, 24
  sub rsp, 24
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 24
  mov rax, [rsp+0]
  mov [rsp+8], rax
  mov rax, 10
  sub rsp, 24
  mov rbx, [rsp+32]
  mov [rsp+0], rbx
  mov [rsp+8], rax
  mov [rsp+16], rdi
  call lookup
  mov rdi, [rsp+16]
  add rsp, 24
  sub rsp, 24
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 24
  mov rax, [rsp+0]
  mov [rsp+8], rax
  mov rax, 12
  sub rsp, 24
  mov rbx, [rsp+32]
  mov [rsp+0], rbx
  mov [rsp+8], rax
  mov [rsp+16], rdi
  call lookup
  mov rdi, [rsp+16]
  add rsp, 24
  sub rsp, 24
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 24
  mov rax, [rsp+0]
  mov [rsp+8], rax
  mov rax, 24
  sub rsp, 24
  mov rbx, [rsp+32]
  mov [rsp+0], rbx
  mov [rsp+8], rax
  mov [rsp+16], rdi
  call lookup
  mov rdi, [rsp+16]
  add rsp, 24
  sub rsp, 24
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 24
  mov rax, [rsp+0]
  mov [rsp+8], rax
  mov rax, 60
  sub rsp, 24
  mov rbx, [rsp+32]
  mov [rsp+0], rbx
  mov [rsp+8], rax
  mov [rsp+16], rdi
  call lookup
  mov rdi, [rsp+16]
  add rsp, 24
  sub rsp, 24
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 24
  mov rax, [rsp+0]
  mov [rsp+8], rax
  mov rax, 2000
  sub rsp, 24
  mov rbx, [rsp+32]
  mov [rsp+0], rbx
  mov [rsp+8], rax
  mov [rsp+16], rdi
  call lookup
  mov rdi, [rsp+16]
  add rsp, 24
  sub rsp, 24
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 24
  mov rax, [rsp+0]
  mov [rsp+8], rax
  mov rax, 486
  sub rsp, 24
  mov rbx, [rsp+32]
  mov [rsp+0], rbx
  mov [rsp+8], rax
  mov [rsp+16], rdi
  call lookup
  mov rdi, [rsp+16]
  add rsp, 24
  sub rsp, 24
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 24
  mov rax, [rsp+0]
  mov [rsp+8], rax
  mov rax, 40
  sub rsp, 24
  mov rbx, [rsp+32]
  mov [rsp+0], rbx
  mov [rsp+8], rax
  mov [rsp+16], rdi
  call lookup
  mov rdi, [rsp+16]
  add rsp, 24
  sub rsp, 24
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 24
  mov rax, [rsp+0]
  mov [rsp+8], rax
  mov rax, 1000
  sub rsp, 24
  mov rbx, [rsp+32]
  mov [rsp+0], rbx
  mov [rsp+8], rax
  mov [rsp+16], rdi
  call lookup
  mov rdi, [rsp+16]
  add rsp, 24
  sub rsp, 24
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 24
  mov rax, [rsp+0]
  sub rsp, 24
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 24
  add rsp, 64
  ret
