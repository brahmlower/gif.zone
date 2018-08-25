
import React, { Component } from 'react';
import { Segment, Container, Grid, List, Header } from 'semantic-ui-react';

export class SiteFooter extends Component {
  render() {
    return (
      <Segment vertical>
        <Container>
          <Grid stackable columns={2}>
            <Grid.Row>
              <Grid.Column>
                <List link>
                  <Header> Development </Header>
                  <List.Item as='a' href='/about'>About</List.Item>
                  <List.Item as='a' href='http://brahmlower.io/tag/gifzone.html'> Blog </List.Item>
                  <List.Item as='a' href='/todo'> Todo </List.Item>
                  <List.Item as='a' href='https://github.com/bplower/gif.zone'> Source </List.Item>
                </List>
              </Grid.Column>
            </Grid.Row>
          </Grid>
        </Container>
      </Segment>
    )
  }
}