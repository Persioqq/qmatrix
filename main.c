#include "qmatrix.h"
	
int
main(void)
{
	printf("Persioqq\n");

	qmatrix *m1 = new_m(10, 10);
	print_m(m1);
	fill_m_with_c(m1);
	print_m(m1);

	qmatrix *m2 = new_m(10, 10);
	print_m(m2);
	fill_m_with_c(m2);
	print_m(m2);

	qmatrix *res = mul_m(m1, m2);
	print_m(res);

	res = add_m(res, m1);
	print_m(res);

	res = sub_m(res, m2);
	print_m(res);

	res = pow_m(res, 2);
	print_m(res);

	free_m(m1);
	free_m(m2);
	free_m(res);

	return 0;
}
