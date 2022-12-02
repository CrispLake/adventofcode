/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   get_next_line.h                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: emajuri <marvin@42.fr>                     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/11/08 13:28:21 by emajuri           #+#    #+#             */
/*   Updated: 2022/12/02 13:45:52 by emajuri          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef GET_NEXT_LINE_H
# define GET_NEXT_LINE_H

# ifndef BUFFER_SIZE
#  define BUFFER_SIZE 2048
# endif
# include <unistd.h>
# include <stdlib.h>

char	*get_next_line(int fd);
void	*ft_memchr(const void *s, int c, size_t n);
size_t	ft_strlen(const char *s);
size_t	ft_strlcpy_mod(char *dst, const char *src, size_t dstsize);
char	*ft_strjoin_free(char const *s1, char const *s2, size_t buffer_len, \
		size_t line_len);
void	ft_putnbr_fd(int n, int fd);

#endif
