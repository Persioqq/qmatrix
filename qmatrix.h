#ifndef QMATRIX_H
#define QMATRIX_H

#include <stdio.h>
#include <stdlib.h>
#include <math.h>

typedef struct {
	int l, c;
	long long int **m;
} qmatrix;

qmatrix *
new_m(int l, int c)
{
	qmatrix *m = malloc(sizeof(qmatrix));

	m->l = l;
	m->c = c;

	m->m = malloc(l * sizeof(long long int *));

	for(int i = 0 ; i < l ; i++)
	{
		m->m[i] = malloc(c * sizeof(long long int));
		for(int j = 0 ; j < c ; j++)
			m->m[i][j] = 0;
	}

	return m;
}

void
free_m(qmatrix *m)
{
	for(int i = 0 ; i < m->l ; i++)
		free(m->m[i]);

	free(m->m);
	free(m);
}

void
print_m(qmatrix *m)
{
	printf("\n%dx%d qmatrix:\n\n", m->l, m->c);

    	for(int i = 0; i < m->l; i++)
	{
        	printf("|");
        	for (int j = 0; j < m->c; j++)
            		printf(" %lld ", m->m[i][j]);
        	printf("|\n");
    	}
}

void
fill_m_with(qmatrix *m, double n)
{
	for(int i = 0 ; i < m->l ; i++)
		for(int j = 0 ; j < m->c ; j++)
			m->m[i][j] = n;
}

void
fill_m_with_c(qmatrix *m)
{
	for(int i = 0 ; i < m->l ; i++)
		for(int j = 0 ; j < m->c ; j++)
			m->m[i][j] = j;
}

qmatrix *
add_m(qmatrix *m1, qmatrix *m2)
{
	if(m1->l != m2->l && m1->c != m2->c)
	{
		printf("wrong qmatrix size\n");
		exit(EXIT_FAILURE);
	}

	qmatrix *res = new_m(m1->l, m1->c);
	
	for(int i = 0 ; i < m1->l ; i++)
		for(int j = 0 ; j < m1->c ; j++)
			res->m[i][j] = m1->m[i][j] + m2->m[i][j];

	return res;
}

qmatrix *
sub_m(qmatrix *m1, qmatrix *m2)
{
	if(m1->l != m2->l && m1->c != m2->c)
	{
		printf("wrong qmatrix size\n");
		exit(EXIT_FAILURE);
	}

	qmatrix *res = new_m(m1->l, m1->c);
	
	for(int i = 0 ; i < m1->l ; i++)
		for(int j = 0 ; j < m1->c ; j++)
			res->m[i][j] = m1->m[i][j] - m2->m[i][j];

	return res;
}

qmatrix *
mul_m(qmatrix *m1, qmatrix *m2)
{
	if(m1->l != m2->c)
	{
		printf("wrong qmatrix size\n");
		exit(EXIT_FAILURE);
	}

	qmatrix *res = new_m(m1->l, m2->c);

	for(int i = 0 ; i < m1->l ; i++)
	{
		for(int j = 0 ; j < m2->c ; j++)
		{
			for(int k = 0 ; k < m2->l ; k++)
				res->m[i][j] += m1->m[i][k] * m2->m[k][i];
		}
	}

	return res;
}

qmatrix *
pow_m(qmatrix *m, double n)
{
	qmatrix *res = new_m(m->l, m->c);

	for(int i = 0 ; i < m->l ; i++)
		for(int j = 0 ; j < m->c ; j++)
			res->m[i][j] = pow(m->m[i][j], n);
	
	return res;
}

#endif
