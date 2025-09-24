#include <stdio.h>
#include <stdlib.h>
#include "calculator.h"



void main(){

	char oper;
	int num1,num2,result;

	start:
	printf("Enter an operation to use (+,-,*,/): ");
	scanf(" %c", &oper);

	printf("Enter first number for calculations: ");
	scanf(" %d", &num1);

	printf("Enter second number for calculations: ");
	scanf(" %d", &num2);

	switch(oper){
		case '+':
			result = addition(num1, num2);
			break;
		case '-':
			result = subtraction(num1, num2);
			break;
		case '*':
			result = multiplication(num1, num2);
			break;
		case '/':
			result = division(num1, num2);
			break;
		default:
			printf("Invalid operation\n");
			goto start;

	}

	printf("Result is: %d\n",result);
}
