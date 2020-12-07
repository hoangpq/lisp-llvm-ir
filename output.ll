; ModuleID = 'main_module'
source_filename = "main_module"

@0 = private constant [3 x i8] c"%u\00"
@1 = private constant [3 x i8] c"%u\00"
@2 = private constant [12 x i8] c"Result: %d\0A\00"

declare i8 @printf(...)

declare i32 @scanf(...)

define i32 @main() {
entry:
  %input = alloca i32, align 4
  %0 = call i32 (...) @scanf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @0, i32 0, i32 0), i32* %input)
  %input1 = alloca i32, align 4
  %1 = call i32 (...) @scanf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @1, i32 0, i32 0), i32* %input1)
  %2 = alloca i32, align 4
  store i32 20, i32* %2, align 4
  %3 = load i32, i32* %input1, align 4
  %4 = load i32, i32* %2, align 4
  %add_ret = add i32 %3, %4
  %5 = load i32, i32* %input, align 4
  %sub_ret = sub i32 %5, %add_ret
  %6 = alloca i32, align 4
  store i32 %sub_ret, i32* %6, align 4
  %7 = load i32, i32* %6, align 4
  %8 = call i8 (...) @printf(i8* getelementptr inbounds ([12 x i8], [12 x i8]* @2, i32 0, i32 0), i32 %7)
  ret i32 %sub_ret
}
