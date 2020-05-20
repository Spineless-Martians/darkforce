
module.exports = {
  siteMetadata: {
    title: `Darkforce`,
    description: `Darkforce`,
    author: `Joatin Granlund <granlundjoatin@icloud.com>, lykata`,
    repository: `https://github.com/Spineless-Martians/darkforce`,
    menuLinks:[
      {
        name:'Schemas',
        link:'/schemas',
        icon: 'book'
      },
      {
        name:'Stats',
        link:'/stats',
        icon: 'rss'
      },
      {
        name:'Settings',
        link:'/settings',
        icon: 'rss'
      }
    ]
  },
  plugins: [
    `gatsby-plugin-react-helmet`,
    {
      resolve: `gatsby-source-filesystem`,
      options: {
        name: `images`,
        path: `${__dirname}/src/images`,
      },
    },
    `gatsby-transformer-sharp`,
    `gatsby-plugin-sharp`,
    `gatsby-plugin-typescript`,
    `gatsby-plugin-sass`,
    {
      resolve: `gatsby-plugin-manifest`,
      options: {
        name: `darkforce`,
        short_name: `darkforce`,
        start_url: `/`,
        background_color: `#663399`,
        theme_color: `#663399`,
        display: `minimal-ui`,
        icon: `src/images/gatsby-icon.png`, // This path is relative to the root of the site.
      },
    },
    `gatsby-plugin-offline`
  ],
};
