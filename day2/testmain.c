/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   testmain.c                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: emajuri <marvin@42.fr>                     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/11/08 13:17:37 by emajuri           #+#    #+#             */
/*   Updated: 2022/12/02 14:11:48 by emajuri          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "get_next_line.h"
#include <fcntl.h>
#include <stdio.h>

int	main(int argc, char **argv)
{
	int fd;
	char	*line;
	int score = 0;
	char elf;
	char me;

	argc += 0;
	fd = open(argv[1], O_RDONLY);
	if (fd < 0)
		return (0);
	while ((line = get_next_line(fd)))
	{
		printf("%s", line);
		elf = line[0];
		me = line[2];
		free(line);
		if (elf == 'A')
		{
			if (me == 'X')
				score += 3;
			else if (me == 'Y')
				score += 4;
			else
				score += 8;
		}
		else if (elf == 'B')
		{
			if (me == 'X')
				score += 1;
			else if (me == 'Y')
				score += 5;
			else
				score += 9;
		}
		else if (elf == 'C')
		{
			if (me == 'X')
				score += 2;
			else if (me == 'Y')
				score += 6;
			else
				score += 7;
		}
	}
	close(fd);
	printf("\n\n");
	ft_putnbr_fd(score, 1);
	return (0);
}
