import React, {Component} from "react";
import styles from './Menu.module.scss';
import {graphql, Link, StaticQuery} from "gatsby";

const QUERY = graphql`
          query MenuQuery {
            site {
              siteMetadata {
                menuItems {
                  name
                  link
                  icon
                }
              }
            }
          }
        `;


export default class Menu extends Component {

  render() {
    return (
      <div className={styles.menu}>
        <StaticQuery
          query={QUERY}
          render={({site}) => {
            const items = site.siteMetadata.menuItems;
            return (
              <>
                { items.map((i) => (
                  <Link to={i.link} activeClassName={styles.active} className={styles.item}>
                    <span>{i.name}</span>
                  </Link>
                )) }
              </>
            );
          }}
        />
        <a href={'/playground'} className={styles.item}>
          <span>GraphQL Api</span>
        </a>
      </div>
    );
  }
}
