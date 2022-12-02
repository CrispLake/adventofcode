/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   testmain.c                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: emajuri <marvin@42.fr>                     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/11/08 13:17:37 by emajuri           #+#    #+#             */
/*   Updated: 2022/12/02 13:30:01 by emajuri          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "get_next_line.h"
#include <fcntl.h>
#include <stdio.h>

int	main(int argc, char **argv)
{
	int fd;
	char	*line;
	int count = 0;
	int highest = 0;
	int	second = 0;
	int third = 0;

	argc += 0;
	fd = open(argv[1], O_RDONLY);
	if (fd < 0)
		return (0);
	while ((line = get_next_line(fd)))
	{
		printf("%s", line);
		if (*line != '\n')
			count += ft_atoi(line);
		else
		{
			if (highest < count)
			{
				third = second;
				second = highest;
				highest = count;
			}
			else if (second < count)
			{
				third = second;
				second = count;
			}
			else if (third < count)
				third = count;
			count = 0;
		}
		free(line);
	}
	close(fd);
	printf("\n\n");
	ft_putnbr_fd(highest, 1);
	printf("\n\n");
	highest += second + third;
	ft_putnbr_fd(highest, 1);
	return (0);
}
