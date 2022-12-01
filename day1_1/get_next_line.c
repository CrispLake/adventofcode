/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   get_next_line.c                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: emajuri <marvin@42.fr>                     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/11/15 16:04:41 by emajuri           #+#    #+#             */
/*   Updated: 2022/11/18 17:42:27 by emajuri          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "get_next_line.h"

static char	*read_line(char *line, int fd, size_t *line_len)
{
	char	*buffer;
	int		read_value;

	read_value = 1;
	while (!ft_memchr(line, '\n', *line_len) && read_value)
	{
		buffer = malloc(BUFFER_SIZE * sizeof(char));
		if (!buffer)
			return (NULL);
		read_value = read(fd, buffer, BUFFER_SIZE);
		if (read_value < 0)
		{
			free(buffer);
			return (NULL);
		}
		line = ft_strjoin_free(line, buffer, read_value, *line_len);
		*line_len += read_value;
		if (!line)
			return (NULL);
	}
	return (line);
}

static size_t	return_size(char *line)
{
	size_t	i;

	i = 0;
	while (line[i] != '\n')
	{
		i++;
		if (line[i] == '\0')
		{
			i--;
			break ;
		}
	}
	return (i);
}

static char	*get_line(char *line, size_t size)
{
	char	*return_line;

	return_line = malloc((size + 2) * sizeof(char));
	if (!return_line)
		return (NULL);
	ft_strlcpy_mod(return_line, line, size + 2);
	return (return_line);
}

char	*delete_old_line(char *line, size_t *line_len)
{
	char	*new;
	size_t	len;
	size_t	i;

	i = 0;
	len = 0;
	while (line[i] != '\n' && line[i] != '\0')
	{
		i++;
		len++;
	}
	if (line[i] == '\n')
		len++;
	*line_len -= len;
	new = malloc((*line_len + 1) * sizeof(char));
	if (!new)
	{
		free(line);
		return (NULL);
	}
	ft_strlcpy_mod(new, &line[len], (*line_len) + 1);
	free(line);
	line = NULL;
	return (new);
}

char	*get_next_line(int fd)
{
	static char		*line;
	char			*return_line;
	static size_t	line_len = 0;

	line = read_line(line, fd, &line_len);
	if (!line)
		return (NULL);
	if (*line == '\0')
	{
		free(line);
		line = NULL;
		return (NULL);
	}
	return_line = get_line(line, return_size(line));
	if (return_line)
		line = delete_old_line(line, &line_len);
	return (return_line);
}
