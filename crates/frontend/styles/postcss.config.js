const path = require('path');

const tailwindScreenQueries = () => {

    const buildQuery = (value) => {
        const getMinMaxQuery = (value) => {
        return [
            value.min ? `(min-width: ${value.min})` : null,
            value.max ? `(max-width: ${value.max})` : null,
        ].filter(Boolean).join(' and ')
    }

        const query = typeof value === 'string'
            ? `(min-width: ${value})`
            : Array.isArray(value)
                ? value.map((_value) => buildQuery(_value)).join(', ')
                : value.min || value.max
                    ? getMinMaxQuery(value)
                    : value.raw

        return query
    }

    return Object
        .entries( require('./tailwind.config.js' ).screens )
        .map( ([name, value]) => ({ [buildQuery(value)]: name }) )
        .reduce( (accumulator, value) => Object.assign(accumulator, value) )
}

module.exports = {
    plugins: {
        'postcss-extract-media-query': {
            output: {
                path: path.join(__dirname, './dist'),
            },
            queries: tailwindScreenQueries(),
        },
    },
};
