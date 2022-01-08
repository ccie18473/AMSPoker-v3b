use crate::prelude::*;

pub type Point2 = cgmath::Point2<f32>;
pub type Vector2 = cgmath::Vector2<f32>;

pub const CARD_WIDTH: f32 = 96.0;
pub const CARD_HEIGHT: f32 = 144.0;
pub const COLUMNS: usize = 7;
pub const ROWS: usize = 4;

pub const Y_OFFSET: f32 = 20.0;

pub const VALUE_RUST: usize = 52;

pub struct TRender {
    pub table: TTable,
    pub resources: TResources,
    pub duration: i32,
    pub card_width: f32,
    pub card_height: f32,
}

impl TRender {
    pub fn new_render(ctx: &mut Context) -> TRender {
        let s = Self {
            table: TTable::new_table(ctx),
            resources: TResources::new_resources(ctx),
            duration: 0,
            card_width: 0.0,
            card_height: 0.0,
        };
        s
    }
    pub fn table_render(&mut self, ctx: &mut Context) -> GameResult {
        // Table background image
        let scale = Vector2::new(
            (self.card_width / CARD_WIDTH) as f32,
            (self.card_height / CARD_HEIGHT) as f32,
        );
        let p = graphics::DrawParam::new()
            .dest(Point2::new(0.0, 0.0))
            .scale(scale);
        graphics::draw(ctx, &self.table.image, p)?;

        Ok(())
    }
    pub fn deck_render(&mut self, ctx: &mut Context) -> GameResult {
        // Deck card image
        let scale = Vector2::new(
            (self.card_width / CARD_WIDTH) as f32,
            (self.card_height / CARD_HEIGHT) as f32,
        );

        let p = graphics::DrawParam::new()
            .dest(Point2::new(0.0, 0.0))
            .scale(scale);
        graphics::draw(
            ctx,
            &self.resources.get_image(self.table.game.deck.image),
            p,
        )?;

        Ok(())
    }
    pub fn game_refresh(&mut self, ctx: &mut Context) -> GameResult {
        // Double card image
        let scale = Vector2::new(
            (self.card_width / CARD_WIDTH) as f32,
            (self.card_height / CARD_HEIGHT) as f32,
        );

        let p = graphics::DrawParam::new()
            .dest(Point2::new(6.0 * self.card_width, 0.0))
            .scale(scale);
        graphics::draw(ctx, &self.resources.get_image(VALUE_RUST), p)?;

        let mut i = 1.0;
        for card in self.table.game.hand.cards.iter() {
            let p = graphics::DrawParam::new()
                .dest(Point2::new(self.card_width * i, 0.0))
                .scale(scale);
            graphics::draw(ctx, &self.resources.get_image(card.image), p)?;
            i += 1.0;
        }

        Ok(())
    }
    pub fn game_double(&mut self, ctx: &mut Context) -> GameResult {
        let scale = Vector2::new(
            (self.card_width / CARD_WIDTH) as f32,
            (self.card_height / CARD_HEIGHT) as f32,
        );

        let p = graphics::DrawParam::new()
            .dest(Point2::new(6.0 * self.card_width, 0.0))
            .scale(scale);
        graphics::draw(
            ctx,
            &self.resources.get_image(self.table.game.card_double.image),
            p,
        )?;

        Ok(())
    }
    pub fn panel_render(&mut self, ctx: &mut Context) -> GameResult {
        let prizes = [
            "Royal Flush",
            "Straight Flush",
            "Four of a Kind",
            "Full House",
            "Flush",
            "Straight",
            "Tree of a Kind",
            "Two Pair",
            "Pair of Aces",
        ];

        let status = ["Wins:", "Credits:", "Bets"];

        for i in 0..9 {
            let p = cgmath::Point2::new(0.0, self.card_height + Y_OFFSET * ((i as f32) + 1.0));
            let text = graphics::Text::new(prizes[i]);
            graphics::draw(ctx, &text, (p,))?;
        }

        for i in 0..3 {
            let p = cgmath::Point2::new(
                4.0 * self.card_width,
                self.card_height + Y_OFFSET * ((i as f32) * 2.0 + 1.0),
            );
            let text = graphics::Text::new(status[i]);
            graphics::draw(ctx, &text, (p,))?;
        }

        Ok(())
    }

    pub fn panel_refresh(&mut self, ctx: &mut Context) -> GameResult {
        let prizes = [500, 80, 25, 10, 8, 5, 3, 2, 1];

        for i in 0..9 {
            let p = cgmath::Point2::new(
                2.0 * self.card_width,
                self.card_height + Y_OFFSET * ((i as f32) + 1.0),
            );
            let text = graphics::Text::new((self.table.game.panel.value * prizes[i]).to_string());
            graphics::draw(ctx, &text, (p,))?;
        }

        let p = cgmath::Point2::new(5.0 * self.card_width, self.card_height + Y_OFFSET);
        let text = graphics::Text::new(self.table.game.panel.wins.to_string());
        graphics::draw(ctx, &text, (p,))?;
        let p = cgmath::Point2::new(5.0 * self.card_width, self.card_height + Y_OFFSET * 3.0);
        let text = graphics::Text::new(self.table.game.panel.credits.to_string());
        graphics::draw(ctx, &text, (p,))?;
        let p = cgmath::Point2::new(5.0 * self.card_width, self.card_height + Y_OFFSET * 5.0);
        let text = graphics::Text::new(self.table.game.panel.bets.to_string());
        graphics::draw(ctx, &text, (p,))?;

        let p = cgmath::Point2::new(4.0 * self.card_width, self.card_height + Y_OFFSET * 7.0);
        let text = graphics::Text::new(self.table.game.panel.msg.to_string());
        graphics::draw(ctx, &text, (p,))?;

        let p = cgmath::Point2::new(4.0 * self.card_width, self.card_height + Y_OFFSET * 9.0);
        let text = graphics::Text::new(self.table.game.panel.prize.to_string());
        graphics::draw(ctx, &text, (p,))?;

        Ok(())
    }
    pub fn button_render(&mut self, ctx: &mut Context) -> GameResult {
        let scale = Vector2::new(
            (self.card_width / CARD_WIDTH) as f32,
            (self.card_height / CARD_HEIGHT) as f32,
        );

        for i in 0..7 {
            let p = graphics::DrawParam::new()
                .dest(Point2::new(
                    (i as f32) * self.card_width,
                    3.0 * self.card_height,
                ))
                .scale(scale);
            graphics::draw(
                ctx,
                &self
                    .resources
                    .get_image(self.table.button_set.buttons[i].image),
                p,
            )?;
        }

        Ok(())
    }
    pub fn fps_render(&mut self, ctx: &mut Context) -> GameResult {
        let fps = timer::fps(ctx);
        let fps_display = graphics::Text::new(format!("FPS: {:.2}", fps));
        let p = cgmath::Point2::new(3.0 * self.card_width, self.card_height + Y_OFFSET * 11.0);
        graphics::draw(ctx, &fps_display, (p,))?;

        Ok(())
    }
    pub fn duration_render(&mut self, ctx: &mut Context) -> GameResult {
        let secs_display = graphics::Text::new(format!("Duration: {}", self.duration));
        let p = cgmath::Point2::new(3.0 * self.card_width, self.card_height + Y_OFFSET * 12.0);
        graphics::draw(ctx, &secs_display, (p,))?;

        Ok(())
    }
    pub fn init(&mut self, ctx: &mut Context) -> GameResult {
        self.table_render(ctx).unwrap();
        self.deck_render(ctx).unwrap();
        self.panel_render(ctx).unwrap();
        self.button_render(ctx).unwrap();
        self.fps_render(ctx).unwrap();
        self.duration_render(ctx).unwrap();

        Ok(())
    }
    pub fn display(&mut self, ctx: &mut Context) -> GameResult {
        // Table, Deck, Panel and Button initial state
        self.init(ctx).unwrap();
        // Bet, Panel refresh
        if self.table.game.state == 1 {
            self.panel_refresh(ctx).unwrap();
        // Game refresh, Panel refresh
        } else if self.table.game.state == 2
            || self.table.game.state == 3
            || self.table.game.state == 4
        {
            self.game_refresh(ctx).unwrap();
            self.panel_refresh(ctx).unwrap();
        // Red/Black, Panel Refresh, Double
        } else if self.table.game.state == 5 {
            self.game_double(ctx).unwrap();
            self.panel_refresh(ctx).unwrap();
        }

        Ok(())
    }
}
