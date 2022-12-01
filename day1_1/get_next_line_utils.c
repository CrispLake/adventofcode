/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   get_next_line_utils.c                              :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: emajuri <marvin@42.fr>                     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/11/14 13:08:52 by emajuri           #+#    #+#             */
/*   Updated: 2022/11/17 15:26:59 by emajuri          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "get_next_line.h"

void	*ft_memchr(const void *s, int c, size_t n)
{
	if (!s)
		return (NULL);
	while (n-- > 0)
		if (*(const unsigned char *)s++ == (unsigned char)c)
			return ((void *)--s);
	return (NULL);
}

size_t	ft_strlen(const char *s)
{
	size_t	i;

	i = 0;
	while (*s++)
		i++;
	return (i);
}

size_t	ft_strlcpy_mod(char *dst, const char *src, size_t dstsize)
{
	while (dstsize-- > 1 && *src != 0)
		*dst++ = *src++;
	*dst = 0;
	return (0);
}

char	*ft_strdup(const char *s1)
{
	char	*ptr;
	size_t	slen;

	slen = ft_strlen(s1);
	ptr = malloc((slen + 1) * sizeof(char));
	if (ptr)
	{
		ft_strlcpy_mod(ptr, s1, slen + 1);
		return (ptr);
	}
	return (NULL);
}

char	*ft_strjoin_free(char const *s1, char const *s2, size_t buffer_len, \
		size_t line_len)
{
	size_t	sumlen;
	size_t	s1len;
	size_t	s2len;
	char	*s;

	if (!s1)
		s1 = ft_strdup("");
	if (!s1 || !s2)
		return (NULL);
	s1len = line_len;
	s2len = buffer_len;
	sumlen = s1len + s2len;
	s = malloc((sumlen + 1) * sizeof(char));
	if (s)
	{
		ft_strlcpy_mod(s, s1, (s1len + 1));
		ft_strlcpy_mod((s + s1len), s2, (s2len + 1));
		free((void *)s1);
		free((void *)s2);
		return (s);
	}
	return (NULL);
}
